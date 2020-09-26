use tokio::process::Command;
use anyhow::{Result, Context, Error};
use async_trait::async_trait;

use crate::{
    services::model::{Nameable, Ensurable, Removable},
    helpers::ExitStatusIntoUnit
};

static NAME: &str = "k3d cluster";
static CLUSTER_NAME: &str = "kfp-local";

#[derive(Default)]
pub struct K3dService {}

impl Nameable for K3dService {
    fn name(&self) -> &'static str {
        NAME
    }
}

#[async_trait]
impl Ensurable for K3dService {
    async fn is_present(&self) -> Result<bool> {
        let ps_out = Command::new("docker")
            .arg("ps")
            .arg("--filter")
            .arg(format!("name={}", CLUSTER_NAME))
            .output().await?.stdout;
        let ps_out_str = std::str::from_utf8(&ps_out)?;

        // TODO: This should probably overwrite the kubeconfig?
        // Or, do kubeconfig management throughout this?

        Ok(ps_out_str.contains(CLUSTER_NAME))
    }

    async fn make_present(&self) -> Result<()> {
        Command::new("k3d")
            .arg("cluster")
            .arg("create")
            .arg(CLUSTER_NAME)
            .status().await
            .status_to_unit()
            .context("Unable to start the k3d k8s cluster.")?;

        println!("Waiting for traefik deployment to complete ...");

        Command::new("kubectl")
            .arg("wait")
            .arg("--for=condition=complete")
            .arg("--timeout=600s")
            .arg("job/helm-install-traefik")
            .arg("-n")
            .arg("kube-system")
            .status().await
            .status_to_unit()
            .context("Unable to wait for the traefik deployment to complete.")?;

        println!("Waiting for traefik deployment to come up ...");

        Command::new("kubectl")
            .arg("wait")
            .arg("--for=condition=available")
            .arg("--timeout=600s")
            .arg("deploy/traefik")
            .arg("-n")
            .arg("kube-system")
            .status().await
            .status_to_unit()
            .context("Unable to wait for the traefik deployment to come up.")?;

        if self.is_present().await? {
            Ok(())
        } else {
            Err(Error::msg("Unable to verify that the k3d cluster is running."))
        }
    }
}

#[async_trait]
impl Removable for K3dService {
    async fn make_not_present(&self) -> Result<()> {
        Command::new("k3d")
            .arg("cluster")
            .arg("delete")
            .arg(CLUSTER_NAME)
            .status().await
            .status_to_unit()
            .context("Unable to stop the k3d k8s cluster.")?;

        Ok(())
    }
}
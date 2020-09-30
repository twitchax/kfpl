use tokio::process::Command;
use anyhow::{Result, Context, Error};
use async_trait::async_trait;

use crate::{
    services::model::{Nameable, Ensurable},
    helpers::ExitStatusIntoUnit
};

static NAME: &str = "KF Service";
// TODO: Fix this.
static SERVICE_NAME: &str = "ml-pipeline";
static TEMP_FOLDER: &str = "kftemp409231";

#[derive(Default)]
pub struct KfService {}

impl Nameable for KfService {
    fn name(&self) -> &'static str {
        NAME
    }
}

#[async_trait]
impl Ensurable for KfService {
    async fn is_present(&self) -> Result<bool> {
        let k_out = Command::new("kubectl")
            .arg("get")
            .arg("pods")
            .arg("--all-namespaces")
            .output().await?.stdout;
        let k_out_str = std::str::from_utf8(&k_out)?;

        Ok(k_out_str.contains(SERVICE_NAME))
    }

    async fn make_present(&self) -> Result<()> {
        Command::new("mkdir")
            .arg("-p")
            .arg(TEMP_FOLDER)
            .status().await
            .status_to_unit()
            .context("Unable to create a KubeFlow temp directory.")?;

        Command::new("sh")
            .arg("-c")
            .arg(format!("cd {}; kfctl apply -V -f https://raw.githubusercontent.com/kubeflow/manifests/v1.1-branch/kfdef/kfctl_k8s_istio.v1.1.0.yaml", TEMP_FOLDER))
            .status().await
            .status_to_unit()
            .context("Unable to apply the KF kustomize script.")?;

        tokio::time::delay_for(tokio::time::Duration::from_secs(10)).await;

        println!("Waiting for the ml-pipeline deployment to complete ...");

        Command::new("kubectl")
            .arg("wait")
            .arg("--for=condition=available")
            .arg("--timeout=600s")
            .arg("deploy/ml-pipeline")
            .arg("-n")
            .arg("kubeflow")
            .status().await
            .status_to_unit()
            .context("Unable to wait for the ml-pipeline deployment to come up.")?;

        Command::new("rm")
            .arg("-rf")
            .arg(TEMP_FOLDER)
            .status().await
            .status_to_unit()
            .context("Unable to remove the KF temp directory.")?;

        if self.is_present().await? {
            Ok(())
        } else {
            Err(Error::msg("Unable to verify that the kf service is running."))
        }
    }
}
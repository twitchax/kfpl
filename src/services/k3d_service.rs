use tokio::process::Command;
use anyhow::{Result, Context, Error};
use async_trait::async_trait;

use crate::{helpers::{self, ExitStatusIntoUnit}, services::model::{Nameable, Ensurable, Removable}};

static NAME: &str = "k3d cluster";

#[derive(Default)]
pub struct K3dService {
    k3d_cluster_name: String,
    k3d_image: String,
    k3d_api_address: String,
    k3d_api_port: String,
}

impl K3dService {
    pub fn with_k3d_cluster_name(mut self, n: &str) -> Self {
        self.k3d_cluster_name = n.to_owned();
        self
    }

    pub fn with_k3d_image(mut self, i: &str) -> Self {
        self.k3d_image = i.to_owned();
        self
    }

    pub fn with_k3d_api_port(mut self, p: &str) -> Self {
        self.k3d_api_port = p.to_owned();
        self
    }

    pub fn with_k3d_api_address(mut self, a: &str) -> Self {
        self.k3d_api_address = a.to_owned();
        self
    }
}

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
            .arg(format!("name={}", self.k3d_cluster_name))
            .output().await?.stdout;
        let ps_out_str = std::str::from_utf8(&ps_out)?;

        // TODO: This should probably overwrite the kubeconfig?
        // Or, do kubeconfig management throughout this?

        Ok(ps_out_str.contains(&self.k3d_cluster_name))
    }

    async fn make_present(&self) -> Result<()> {
        Command::new("k3d")
            .arg("cluster")
            .arg("create")
            .arg(&self.k3d_cluster_name)
            .args(&["--image", &self.k3d_image])
            .args(&["--api-port", &format!("{}:{}", self.k3d_api_address, self.k3d_api_port)])
            //.args(&["-p", "5443:443@loadbalancer"])
            //.args(&["-p", "5080:80@loadbalancer"])
            .status().await
            .status_to_unit()
            .context("Unable to start the k3d k8s cluster.")?;

        tokio::time::delay_for(tokio::time::Duration::from_secs(10)).await;

        println!("Checking if we are inside a container ...");
        if helpers::is_docker().await? {
            // On Mac and Windows, we should replace with `host.docker.internal`.  On Linux, people can just run this executable
            // anyway, so bleh.
            println!("Overwriting the kubeconfig since we are inside a container ...");
            Command::new("sed")
                .arg("-i")
                .arg("s/0.0.0.0/host.docker.internal/g")
                .arg("/root/.kube/config")
                .status().await
                .status_to_unit()
                .context("Unable to overwrite the kubeconfig.")?;
        }

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
            .arg(&self.k3d_cluster_name)
            .status().await
            .status_to_unit()
            .context("Unable to stop the k3d k8s cluster.")?;

        Ok(())
    }
}
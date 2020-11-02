use tokio::process::Command;
use anyhow::{Result, Context, Error};
use async_trait::async_trait;

use crate::{
    services::model::{Nameable, Ensurable},
    helpers::ExitStatusIntoUnit
};

static NAME: &str = "KFP Service";
static SERVICE_NAME: &str = "ml-pipeline";

#[derive(Default)]
pub struct KfpService {
    kfp_version: String
}

impl KfpService {
    pub fn with_kfp_version(mut self, n: &str) -> Self {
        self.kfp_version = n.to_owned();
        self
    }
}

impl Nameable for KfpService {
    fn name(&self) -> &'static str {
        NAME
    }
}

#[async_trait]
impl Ensurable for KfpService {
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
        Command::new("kubectl")
            .arg("apply")
            .arg("-k")
            .arg(format!("github.com/kubeflow/pipelines/manifests/kustomize/cluster-scoped-resources?ref={}", self.kfp_version))
            .status().await
            .status_to_unit()
            .context("Unable to apply the KFP cluster scoped resources.")?;

        Command::new("kubectl")
            .arg("wait")
            .arg("--for")
            .arg("condition=established")
            .arg("--timeout=60s")
            .arg("crd/applications.app.k8s.io")
            .status().await
            .status_to_unit()
            .context("Unable to wait for KFP CRD deployment.")?;

        Command::new("kubectl")
            .arg("apply")
            .arg("-k")
            .arg(format!("github.com/kubeflow/pipelines/manifests/kustomize/env/platform-agnostic-pns?ref={}", self.kfp_version))
            .status().await
            .status_to_unit()
            .context("Unable to apply the KFP platform agnostic deployment.")?;

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

        if self.is_present().await? {
            Ok(())
        } else {
            Err(Error::msg("Unable to verify that the kfp service is running."))
        }
    }
}
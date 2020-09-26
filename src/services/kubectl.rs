use tokio::process::Command;
use anyhow::{Result, Context};
use async_trait::async_trait;

use crate::{
    services::model::{Nameable, Ensurable, is_binary_present},
    helpers::ExitStatusIntoUnit
};

static NAME: &str = "kubectl";
static VERSION: &str = "v1.19.2";

#[derive(Default)]
pub struct Kubectl {}

impl Nameable for Kubectl {
    fn name(&self) -> &'static str {
        NAME
    }
}

#[async_trait]
impl Ensurable for Kubectl {
    async fn is_present(&self) -> Result<bool> {
        is_binary_present(self).await
    }

    async fn make_present(&self) -> Result<()> {
        Command::new("curl")
            .arg("-LO")
            .arg(format!("https://storage.googleapis.com/kubernetes-release/release/{}/bin/linux/amd64/kubectl", VERSION))
            .status().await
            .status_to_unit()
            .context("Unable to curl the kubectl binary.")?;
    
        Command::new("chmod")
            .arg("+x")
            .arg("./kubectl")
            .status().await
            .status_to_unit()
            .context("Unable to change executable permissions on the kubectl binary.")?;

        Command::new("mv")
            .arg("./kubectl")
            .arg("/usr/local/bin/kubectl")
            .status().await
            .status_to_unit()
            .context("Unable to copy the kubectl binary (might need sudo).")?;

        Command::new("kubectl")
            .arg("version")
            .arg("--client")
            .status().await
            .status_to_unit()
            .context("Unable to use kubectl after supposed install.")?;

        Ok(())
    }
}


use tokio::process::Command;
use anyhow::{Result, Context};
use async_trait::async_trait;

use crate::{
    services::model::{Nameable, Ensurable, is_binary_present},
    helpers::ExitStatusIntoUnit
};

static NAME: &str = "k3d";

#[derive(Default)]
pub struct K3d {}

impl Nameable for K3d {
    fn name(&self) -> &'static str {
        NAME
    }
}

#[async_trait]
impl Ensurable for K3d {
    async fn is_present(&self) -> Result<bool> {
        is_binary_present(self).await
    }

    async fn make_present(&self) -> Result<()> {
        Command::new("curl")
            .arg("-fsSL")
            .arg("https://raw.githubusercontent.com/rancher/k3d/main/install.sh")
            .arg("-o")
            .arg("k3d-install.sh")
            .status().await
            .status_to_unit()
            .context("Unable to curl the k3d convenience script.")?;
    
        Command::new("bash")
            .arg("k3d-install.sh")
            .status().await
            .status_to_unit()
            .context("Failed to run the k3d install script.")?;

        Command::new("rm")
            .arg("-f")
            .arg("k3d-install.sh")
            .status().await
            .status_to_unit()
            .context("Failed to delete the k3d install script.")?;

        Ok(())
    }
}
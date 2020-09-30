use tokio::process::Command;
use anyhow::{Result, Context};
use async_trait::async_trait;

use crate::{
    services::model::{Nameable, Ensurable, is_binary_present},
    helpers::ExitStatusIntoUnit
};

static NAME: &str = "docker";

#[derive(Default)]
pub struct Docker {}

impl Nameable for Docker {
    fn name(&self) -> &'static str {
        NAME
    }
}

#[async_trait]
impl Ensurable for Docker {
    async fn is_present(&self) -> Result<bool> {
        is_binary_present(self).await
    }

    async fn make_present(&self) -> Result<()> {
        Command::new("curl")
            .arg("-fsSL")
            .arg("https://get.docker.com")
            .arg("-o")
            .arg("get-docker.sh")
            .status().await
            .status_to_unit()
            .context("Unable to curl the docker convenience script.")?;

        Command::new("sh")
            .arg("get-docker.sh")
            .status().await
            .status_to_unit()
            .context("Unable to run the docker install script (might need sudo).")?;

        // Ignore failure: might be in container.
        Command::new("usermod")
            .arg("-aG")
            .arg("docker")
            .arg("$USER")
            .status().await
            .status_to_unit().unwrap_or(());
                
        Command::new("rm")
            .arg("-f")
            .arg("get-docker.sh")
            .status().await
            .status_to_unit()
            .context("Failed to delete the docker install script.")?;

        Ok(())
    }
}
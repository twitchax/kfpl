use tokio::process::Command;
use anyhow::{Result, Context};
use async_trait::async_trait;

use crate::{
    services::model::{Nameable, Ensurable, is_binary_present},
    helpers::ExitStatusIntoUnit
};

static NAME: &str = "pip3";

#[derive(Default)]
pub struct Pip3 {}

impl Nameable for Pip3 {
    fn name(&self) -> &'static str {
        NAME
    }
}

#[async_trait]
impl Ensurable for Pip3 {
    async fn is_present(&self) -> Result<bool> {
        is_binary_present(self).await
    }

    async fn make_present(&self) -> Result<()> {
        Command::new("apt-get")
            .arg("update")
            .status().await
            .status_to_unit()
            .context("Unable to update apt-get.")?;
    
        Command::new("apt-get")
            .arg("-y")
            .arg("install")
            .arg("python3")
            .arg("python3-pip")
            .status().await
            .status_to_unit()
            .context("Unable to install pip3 via apt-get.  You can install python3 and pip3 manually, and try again.")?;

        Command::new("pip3")
            .arg("--version")
            .status().await
            .status_to_unit()
            .context("Unable to verify pip3 installation.")?;

        Ok(())
    }
}


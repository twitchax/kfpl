use tokio::process::Command;
use anyhow::{Result, Context};
use async_trait::async_trait;

use crate::{
    services::model::{Nameable, Ensurable, is_binary_present},
    helpers::ExitStatusIntoUnit
};

static NAME: &str = "git";

#[derive(Default)]
pub struct Git {}

impl Nameable for Git {
    fn name(&self) -> &'static str {
        NAME
    }
}

#[async_trait]
impl Ensurable for Git {
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
            .arg("git")
            .status().await
            .status_to_unit()
            .context("Unable to install git via apt-get.  You can install git manually, and try again.")?;

        Command::new("which")
            .arg("git")
            .status().await
            .status_to_unit()
            .context("Unable to verify git installation.")?;

        Ok(())
    }
}


use tokio::process::Command;
use anyhow::{Result, Context};
use async_trait::async_trait;

use crate::{
    services::model::{Nameable, Ensurable, is_binary_present},
    helpers::ExitStatusIntoUnit
};

static NAME: &str = "curl";

#[derive(Default)]
pub struct Curl {}

impl Nameable for Curl {
    fn name(&self) -> &'static str {
        NAME
    }
}

#[async_trait]
impl Ensurable for Curl {
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
            .arg("curl")
            .status().await
            .status_to_unit()
            .context("Unable to install curl via apt-get.  You can install python3 and pip3 manually, and try `kfpl init` again.")?;

        Command::new("which")
            .arg("curl")
            .status().await
            .status_to_unit()
            .context("Unable to verify curl installation.")?;

        Ok(())
    }
}


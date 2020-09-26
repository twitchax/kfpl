use tokio::process::Command;
use anyhow::{Result, Context};
use async_trait::async_trait;

use crate::{
    services::model::{Nameable, Ensurable, is_binary_present},
    helpers::ExitStatusIntoUnit
};

static NAME: &str = "kfp";

#[derive(Default)]
pub struct Kfp {}

impl Nameable for Kfp {
    fn name(&self) -> &'static str {
        NAME
    }
}

#[async_trait]
impl Ensurable for Kfp {
    async fn is_present(&self) -> Result<bool> {
        is_binary_present(self).await
    }

    async fn make_present(&self) -> Result<()> {
        Command::new("pip3")
            .arg("install")
            .arg("urllib3==1.24.2")
            .arg("kfp")
            .arg("kfp-server-api")
            .arg("--upgrade")
            .arg("--user")
            .status().await
            .status_to_unit()
            .context("Unable to install kfp cli.")?;

        Command::new("sh")
            .arg("-c")
            .arg("cp $HOME/.local/bin/kfp /usr/local/bin/kfp")
            .status().await
            .status_to_unit()
            .context("Unable to copy the kfp binary.")?;

        Command::new("sh")
            .arg("-c")
            .arg("cp $HOME/.local/bin/dsl-compile /usr/local/bin/dsl-compile")
            .status().await
            .status_to_unit()
            .context("Unable to copy the dsl-compile binary.")?;

        Command::new("which")
            .arg("dsl-compile")
            .status().await
            .status_to_unit()
            .context("Unable to verify dsl-compile install.")?;
    
        Command::new("which")
            .arg("kfp")
            .status().await
            .status_to_unit()
            .context("Unable to verify kfp install.")?;

        Ok(())
    }
}


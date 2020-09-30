use tokio::process::Command;
use anyhow::{Result, Context};
use async_trait::async_trait;

use crate::{
    services::model::{Nameable, Ensurable, is_binary_present},
    helpers::ExitStatusIntoUnit
};

static NAME: &str = "k9s";

#[derive(Default)]
pub struct K9s {}

impl Nameable for K9s {
    fn name(&self) -> &'static str {
        NAME
    }
}

#[async_trait]
impl Ensurable for K9s {
    async fn is_present(&self) -> Result<bool> {
        is_binary_present(self).await
    }

    async fn make_present(&self) -> Result<()> {
        Command::new("curl")
            .arg("-LO")
            .arg("https://github.com/derailed/k9s/releases/download/v0.22.1/k9s_Linux_x86_64.tar.gz")
            .status().await
            .status_to_unit()
            .context("Unable to curl the k9s tarball.")?;

        Command::new("tar")
            .arg("-xvf")
            .arg("./k9s_Linux_x86_64.tar.gz")
            .status().await
            .status_to_unit()
            .context("Unable to untar the k9s tarball.")?;

        Command::new("rm")
            .arg("-f")
            .arg("k9s_Linux_x86_64.tar.gz")
            .arg("LICENSE")
            .arg("README.md")
            .status().await
            .status_to_unit()
            .context("Unable to remove the k9s tarball.")?;
    
        Command::new("chmod")
            .arg("+x")
            .arg("./k9s")
            .status().await
            .status_to_unit()
            .context("Unable to change executable permissions on the k9s binary.")?;

        Command::new("mv")
            .arg("./k9s")
            .arg("/usr/local/bin/k9s")
            .status().await
            .status_to_unit()
            .context("Unable to copy the k9s binary (might need sudo).")?;

        Command::new("k9s")
            .arg("version")
            .status().await
            .status_to_unit()
            .context("Unable to use k9s after supposed install.")?;

        Ok(())
    }
}


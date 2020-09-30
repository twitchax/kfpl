use tokio::process::Command;
use anyhow::{Result, Context};
use async_trait::async_trait;

use crate::{
    services::model::{Nameable, Ensurable, is_binary_present},
    helpers::ExitStatusIntoUnit
};

static NAME: &str = "kfctl";

#[derive(Default)]
pub struct Kfctl {}

impl Nameable for Kfctl {
    fn name(&self) -> &'static str {
        NAME
    }
}

#[async_trait]
impl Ensurable for Kfctl {
    async fn is_present(&self) -> Result<bool> {
        is_binary_present(self).await
    }

    async fn make_present(&self) -> Result<()> {
        Command::new("curl")
            .arg("-LO")
            .arg("https://github.com/kubeflow/kfctl/releases/download/v1.1.0/kfctl_v1.1.0-0-g9a3621e_linux.tar.gz")
            .status().await
            .status_to_unit()
            .context("Unable to curl the kfctl tarball.")?;

        Command::new("tar")
            .arg("-xvf")
            .arg("./kfctl_v1.1.0-0-g9a3621e_linux.tar.gz")
            .status().await
            .status_to_unit()
            .context("Unable to untar the kfctl tarball.")?;

        Command::new("rm")
            .arg("-f")
            .arg("./kfctl_v1.1.0-0-g9a3621e_linux.tar.gz")
            .status().await
            .status_to_unit()
            .context("Unable to remove the kfctl tarball.")?;
    
        Command::new("chmod")
            .arg("+x")
            .arg("./kfctl")
            .status().await
            .status_to_unit()
            .context("Unable to change executable permissions on the kfctl binary.")?;

        Command::new("mv")
            .arg("./kfctl")
            .arg("/usr/local/bin/kfctl")
            .status().await
            .status_to_unit()
            .context("Unable to copy the kfctl binary (might need sudo).")?;

        Command::new("kfctl")
            .arg("version")
            .status().await
            .status_to_unit()
            .context("Unable to use kfctl after supposed install.")?;

        Ok(())
    }
}


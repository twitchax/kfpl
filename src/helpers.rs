use anyhow::{Context, Result, anyhow};
use tokio::process::Command;
use std::process::ExitStatus;

pub trait ExitStatusIntoUnit {
    fn status_to_unit(self) -> Result<()>;
}

impl ExitStatusIntoUnit for Result<ExitStatus, std::io::Error> {
    fn status_to_unit(self) -> Result<()> {
        self
            .map(|s| s.success())
            .map_err(|e| e.into())
            .and_then(|s| if s { Ok(()) } else { Err(anyhow!("The exit code of the operation was not successful.")) })
    }
}

pub async fn is_docker() -> Result<bool> {
    let status = Command::new("grep")
        .arg("docker")
        .arg("/proc/1/cgroup")
        .status().await
        .context("Unable to check if inside a docker container.")?;

    println!("{:?}", status);

    Ok(status.success())
}
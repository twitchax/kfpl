use anyhow::{Result, anyhow};
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
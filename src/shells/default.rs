use std::process::{Command, ExitStatus};

use crate::error::{Error, Result};

#[cfg(target_os = "windows")]
pub fn exec(command: &str) -> Result<ExitStatus> {
    match Command::new("cmd").arg("/C").arg(command).status() {
        Ok(status) => Ok(status),
        Err(e) => return Err(Error::CommandExecutionError(e)),
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn exec(command: &str) -> Result<ExitStatus> {
    match Command::new("bash").arg("-c").arg(command).status() {
        Ok(status) => Ok(status),
        Err(e) => return Err(Error::CommandExecutionError(e)),
    }
}

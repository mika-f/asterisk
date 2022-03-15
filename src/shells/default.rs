use std::process::{Command, ExitStatus};

use crate::error::{Error, Result};

#[cfg(target_os = "windows")]
pub fn exec(command: &str) -> Result<ExitStatus> {
    match Command::new("cmd").arg("/C").arg(command).status() {
        Ok(status) => Ok(status),
        Err(e) => return Err(Error::CommandExecutionError(e)),
    }
}

#[cfg(any(linux, macos))]
pub fn exec(command: &str) -> Result<()> {
    match Command::new("bash").arg("-c").arg(command).status() {
        Ok(status) => Ok(()),
        Err(e) => return Err(Error::CommandExecutionError(e)),
    }
}

use std::process::{Command, ExitStatus};

use crate::error::{Error, Result};

pub fn exec(command: &str) -> Result<ExitStatus> {
    match Command::new("zsh").arg("-c").arg(command).status() {
        Ok(status) => Ok(status),
        Err(e) => return Err(Error::CommandExecutionError(e)),
    }
}

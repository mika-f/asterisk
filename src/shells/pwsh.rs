use std::process::{Command, ExitStatus};

use crate::error::{Error, Result};
use crate::shells::ShellExecutable;

pub struct PowerShell;

impl ShellExecutable for PowerShell {
    fn exec(&self, command: &str) -> Result<ExitStatus> {
        match Command::new("pwsh").arg("-Command").arg(command).status() {
            Ok(status) => Ok(status),
            Err(e) => return Err(Error::CommandExecutionError(e)),
        }
    }
}

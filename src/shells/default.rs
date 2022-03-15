use std::process::{Command, ExitStatus};

use crate::error::{Error, Result};
use crate::shells::ShellExecutable;

pub struct DefaultShell;

impl ShellExecutable for DefaultShell {
    #[cfg(target_os = "windows")]
    fn exec(&self, command: &str) -> Result<ExitStatus> {
        match Command::new("cmd").arg("/C").arg(command).status() {
            Ok(status) => Ok(status),
            Err(e) => return Err(Error::CommandExecutionError(e)),
        }
    }

    #[cfg(any(linux, macos))]
    fn exec(&self, command: &str) -> Result<()> {
        match Command::new("bash").arg("-c").arg(command).status() {
            Ok(status) => Ok(()),
            Err(e) => return Err(Error::CommandExecutionError(e)),
        }
    }
}

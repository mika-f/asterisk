use std::process::{Command, ExitStatus};

use crate::error::{Error, Result};
use crate::shells::ShellExecutable;

pub struct Bash;

impl ShellExecutable for Bash {
    fn exec(&self, command: &str) -> Result<ExitStatus> {
        match Command::new("bash").arg("-c").arg(command).status() {
            Ok(status) => Ok(status),
            Err(e) => return Err(Error::CommandExecutionError(e)),
        }
    }
}

use std::process::{Command, ExitStatus};

use crate::error::{Error, Result};
use crate::shells::ShellExecutable;

pub struct Cmd;

impl ShellExecutable for Cmd {
    fn exec(&self, command: &str) -> Result<ExitStatus> {
        match Command::new("cmd").arg("/C").arg(command).status() {
            Ok(status) => Ok(status),
            Err(e) => return Err(Error::CommandExecutionError(e)),
        }
    }
}

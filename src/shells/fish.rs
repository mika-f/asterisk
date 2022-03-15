use std::process::{Command, ExitStatus};

use crate::error::{Error, Result};
use crate::shells::ShellExecutable;

pub struct Fish;

impl ShellExecutable for Fish {
    fn exec(&self, command: &str) -> Result<ExitStatus> {
        match Command::new("fish").arg("-c").arg(command).status() {
            Ok(status) => Ok(status),
            Err(e) => return Err(Error::CommandExecutionError(e)),
        }
    }
}

use std::process::{Command, ExitStatus};

use crate::error::{Error, Result};

pub fn alias(left: &str, right: &str) -> Option<String> {
    Some(format!("alias {}='{}'", left, right))
}

pub fn exec(command: &str) -> Result<ExitStatus> {
    match Command::new("fish").arg("-c").arg(command).status() {
        Ok(status) => Ok(status),
        Err(e) => return Err(Error::CommandExecutionError(e)),
    }
}

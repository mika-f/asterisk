use std::process::{Command, ExitStatus};

use crate::error::{Error, Result};

pub fn alias(left: &str, right: &str) -> Option<String> {
    Some(format!("doskey {}={}", left, right))
}

pub fn exec(command: Vec<String>) -> Result<ExitStatus> {
    match Command::new("cmd").arg("/C").args(command).status() {
        Ok(status) => Ok(status),
        Err(e) => return Err(Error::CommandExecutionError(e)),
    }
}

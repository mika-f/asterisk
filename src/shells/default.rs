use std::process::ExitStatus;

use crate::error::Result;
use crate::shells::*;

#[cfg(target_os = "windows")]
pub fn alias(left: &str, right: &str) -> Option<String> {
    cmd::alias(left, right)
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn alias(left: &str, right: &str) -> Option<String> {
    bash::alias(left, right)
}

#[cfg(target_os = "windows")]
pub fn exec(command: Vec<String>) -> Result<ExitStatus> {
    cmd::exec(command)
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn exec(command: Vec<String>) -> Result<ExitStatus> {
    bash::exec(command)
}

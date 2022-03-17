use std::process::{Command, ExitStatus};

use crate::error::{Error, Result};

pub fn alias(left: &str, right: &str) -> Option<String> {
    let mut alias = Vec::new();
    alias.push(format!("Set-Variable -name AsteriskAlias_{left} -value \"function ast_{left} {{ {right} `$args }}\" -scope global", left=left, right=right));
    alias.push(format!(
        "Get-Variable AsteriskAlias_{left} -ValueOnly | Invoke-Expression",
        left = left
    ));
    alias.push(format!(
        "Remove-Variable AsteriskAlias_{left} -scope global",
        left = left
    ));
    alias.push(format!("Set-Alias {left} ast_{left}", left = left));

    Some(alias.join("\n"))
}

pub fn exec(command: &str) -> Result<ExitStatus> {
    match Command::new("pwsh").arg("-Command").arg(command).status() {
        Ok(status) => Ok(status),
        Err(e) => return Err(Error::CommandExecutionError(e)),
    }
}

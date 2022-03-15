use clap::ArgEnum;
use serde_derive::{Deserialize, Serialize};
use std::process::ExitStatus;
use strum_macros::{Display, EnumIter, EnumString};

use crate::error::Result;

mod bash;
mod cmd;
mod default;
mod fish;
mod pwsh;
mod zsh;

#[derive(
    ArgEnum,
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    EnumString,
    PartialEq,
    Eq,
    Serialize,
)]
pub enum Shell {
    #[strum(serialize = "bash")]
    Bash,

    #[strum(serialize = "cmd")]
    Cmd,

    #[strum(serialize = "default")]
    Default,

    #[strum(serialize = "fish")]
    Fish,

    #[strum(serialize = "pwsh")]
    PowerShell,

    #[strum(serialize = "zsh")]
    Zsh,
}

pub trait ShellExecutable {
    fn exec(&self, command: &str) -> Result<ExitStatus>;
}

impl ShellExecutable for Shell {
    fn exec(&self, command: &str) -> Result<ExitStatus> {
        match self {
            Shell::Bash => bash::exec(command),
            Shell::Cmd => cmd::exec(command),
            Shell::Default => default::exec(command),
            Shell::Fish => fish::exec(command),
            Shell::PowerShell => pwsh::exec(command),
            Shell::Zsh => zsh::exec(command),
        }
    }
}

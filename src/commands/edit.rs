use clap::Parser;
use std::process::Command;

use crate::error::{Error, Result};
use crate::function::Functions;

#[derive(Parser)]
pub struct Args {
    #[clap(long, env = "EDITOR")]
    editor: Option<String>,
}

pub async fn exec(args: Args) -> Result<()> {
    let path = Functions::path()?;
    let mut command = Command::new(args.editor.unwrap());
    command.arg(path);

    match command.status() {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::CommandExecutionError(e)),
    }
}

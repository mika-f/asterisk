use clap::Parser;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::function::Functions;
use crate::shells::Shell;

#[derive(Parser)]
pub struct Args {
    #[clap()]
    shell: Option<String>,
}

pub async fn exec(args: Args) -> Result<()> {
    let shell = args.shell.unwrap_or("default".to_owned());
    let shell = match Shell::from_str(&shell) {
        Ok(shell) => shell,
        Err(_) => return Err(Error::ShellNotFound(shell)),
    };

    let functions = Functions::load()?;

    for function in functions.into_iter() {
        let alias = function.export(shell);
        match alias {
            Some(alias) => println!("{}", alias),
            None => {}
        }
    }

    Ok(())
}

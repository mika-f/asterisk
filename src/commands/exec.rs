use clap::Parser;
use std::process::exit;

use crate::error::Result;
use crate::function::Functions;
use crate::shells::{Shell, ShellExecutable};

#[derive(Parser)]
pub struct Args {
    #[clap()]
    name: String,

    #[clap(multiple_values = true, takes_value = true, last = true)]
    extra: Vec<String>,
}

pub async fn exec(args: Args) -> Result<()> {
    let functions = Functions::load()?;
    let mut splice = 0;
    let function = match functions.get(&args.name) {
        Some(function) => function,
        None => {
            if args.extra.is_empty() {
                // pass-through
                let command = format!("{}", args.name);
                match Shell::Default.exec(vec![command]) {
                    Ok(status) => exit(status.code().unwrap()),
                    Err(e) => return Err(e),
                };
            }

            let subcommand = args.extra.get(0).unwrap();
            match functions.get_wrap(subcommand, &args.name) {
                Some(function) => {
                    splice = 1;
                    function
                }
                None => {
                    // pass-through commands to base
                    let mut command = vec![];
                    command.push(args.name);
                    command.extend(args.extra);

                    match Shell::Default.exec(command) {
                        Ok(status) => exit(status.code().unwrap()),
                        Err(e) => return Err(e),
                    };
                }
            }
        }
    };

    function.execute(args.extra[splice..].to_vec()).await
}

use clap::Parser;
use std::os::unix::process::ExitStatusExt;
use std::process::{exit, ExitStatus};

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
                    Ok(status) => safe_exit(status),
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

                    let mut extras = vec![];
                    for arg in args.extra.iter() {
                        if arg.contains(' ') {
                            extras.push(format!("\"{}\"", arg).to_owned());
                        } else {
                            extras.push(arg.to_owned());
                        }
                    }
                    command.extend(extras);

                    let command = command.join(" ");

                    match Shell::Default.exec(vec![command]) {
                        Ok(status) => safe_exit(status),
                        Err(e) => return Err(e),
                    };
                }
            }
        }
    };

    function.execute(args.extra[splice..].to_vec()).await
}

#[cfg(target_os = "windows")]
fn safe_exit(status: ExitStatus) -> ! {
    match status.code() {
        Some(status) => exit(status),
        None => unreachable!(),
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn safe_exit(status: ExitStatus) -> ! {
    match status.code() {
        Some(status) => exit(status),
        None => exit(status.signal().unwrap()),
    }
}

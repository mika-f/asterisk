use clap::Parser;

use crate::error::{Error, Result};
use crate::function::Functions;

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
                return Err(Error::CommandNotFoundError(args.name));
            }

            let subcommand = args.extra.get(0).unwrap();
            match functions.get_wrap(subcommand, &args.name) {
                Some(function) => {
                    splice = 1;
                    function
                }
                None => return Err(Error::CommandNotFoundError(args.name)),
            }
        }
    };

    function.execute(args.extra[splice..].to_vec()).await
}

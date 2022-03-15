use clap::Parser;

#[derive(Parser)]
pub enum SubCommand {
    #[clap()]
    Add(add::Args),
}

pub mod add;

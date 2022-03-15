use clap::Parser;

#[derive(Parser)]
pub enum SubCommand {
    #[clap()]
    Add(add::Args),

    #[clap()]
    Edit(edit::Args),
}

pub mod add;
pub mod edit;

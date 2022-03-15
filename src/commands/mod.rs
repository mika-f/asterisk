use clap::Parser;

#[derive(Parser)]
pub enum SubCommand {
    #[clap()]
    Add(add::Args),

    #[clap()]
    Edit(edit::Args),

    #[clap()]
    Exec(exec::Args),
}

pub mod add;
pub mod edit;
pub mod exec;

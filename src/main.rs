use clap::Parser;

mod commands;
mod error;
mod function;
mod hooks;
mod prompt;
mod shells;

#[derive(Parser)]
#[clap(author, about, version)]
struct Args {
    #[clap(subcommand)]
    subcommand: commands::SubCommand,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    let result = match args.subcommand {
        commands::SubCommand::Add(args) => commands::add::exec(args).await,
        commands::SubCommand::Edit(args) => commands::edit::exec(args).await,
        commands::SubCommand::Exec(args) => commands::exec::exec(args).await,
    };

    match result {
        Ok(()) => Ok(()),
        Err(e) => Err(Box::from(e)),
    }
}

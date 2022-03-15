use clap::{ArgGroup, Parser};

use crate::error::Result;
use crate::prompt;

#[derive(Parser)]
#[clap(group(ArgGroup::new("non-interactive")
    .args(&["name", "command"])
    .requires_all(&["name", "command"])
    .multiple(true)
    .conflicts_with("is-interactive"))
)]
pub struct Args {
    #[clap(long)]
    name: Option<String>,

    #[clap(long)]
    command: Option<String>,

    #[clap(long)]
    description: Option<String>,

    #[clap(long)]
    wrap: Option<String>,

    #[clap(long)]
    condition: Option<String>,

    #[clap(long = "pre-hook")]
    pre_hook: Option<String>,

    #[clap(long = "post-hook")]
    post_hook: Option<String>,

    #[clap(long = "is-interactive")]
    is_interactive: bool,
}

pub async fn exec(args: Args) -> Result<()> {
    if args.is_interactive || args.name.is_none() {
        return interactive_add().await;
    }

    Ok(())
}

async fn interactive_add() -> Result<()> {
    let name = prompt::text("What is a new command name?")?;
    let command = prompt::editor(&format!("What is command(s) executed in {}?", name))?;
    let description = prompt::optional_text(&format!("What is description in {}?", name))?;

    let additional = prompt::question("Are you provide additional settings?")?;
    if additional {
        let wrap = prompt::optional_text("What this command wrap existing command?")?;
        let condition = prompt::optional_text("What this command enabled on?")?;
    }

    let hooks = prompt::question("Are you want to use command hooks?")?;
    if hooks {
        let pre_hooks =
            prompt::optional_text(&format!("What is commands executed in before {}?", name))?;
        let post_hooks =
            prompt::optional_text(&format!("What is commands executed in after {}?", name))?;
    }

    Ok(())
}

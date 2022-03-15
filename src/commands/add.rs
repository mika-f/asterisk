use clap::{ArgGroup, Parser};

use crate::error::Result;
use crate::function::{Function, Functions};
use crate::hooks::Hooks;
use crate::prompt;
use crate::shells::Shell;

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

    #[clap(long, arg_enum)]
    shell: Option<Shell>,

    #[clap(long = "pre-hook")]
    pre_hook: Option<String>,

    #[clap(long = "post-hook")]
    post_hook: Option<String>,

    #[clap(long, use_value_delimiter = true)]
    tags: Vec<String>,

    #[clap(long = "is-interactive")]
    is_interactive: bool,
}

pub async fn exec(args: Args) -> Result<()> {
    if args.is_interactive || args.name.is_none() {
        return interactive_add().await;
    } else {
        return arguments_add(args).await;
    }
}

async fn interactive_add() -> Result<()> {
    let mut functions = Functions::load()?;

    let name = prompt::text("What is a new command name?")?;
    let command = prompt::editor(&format!("What is command(s) executed in {}?", name))?;
    let description = prompt::optional_text(&format!("What is description in {}?", name))?;

    let mut function = Function::new(&name, &command);
    function.set_description(description);

    let additional = prompt::question("Are you provide additional settings?")?;
    if additional {
        let wrap = prompt::optional_text("What this command wrap existing command?")?;
        let condition = prompt::optional_text("What this command enabled on?")?;
        let shell = prompt::select::<Shell>("Which shell is used?")?;

        function.set_wrap(wrap);
        function.set_condition(condition);
        function.set_shell(Some(shell));
    }

    let hooks = prompt::question("Are you want to use command hooks?")?;
    if hooks {
        let pre = prompt::optional_text(&format!("What is commands executed in before {}?", name))?;
        let post = prompt::optional_text(&format!("What is commands executed in after {}?", name))?;

        let hooks = Hooks::new(pre, post);
        function.set_hooks(Some(hooks));
    }

    functions.append(function);
    functions.save()?;

    Ok(())
}

async fn arguments_add(args: Args) -> Result<()> {
    let mut functions = Functions::load()?;

    let mut function = Function::new(&args.name.unwrap(), &args.command.unwrap());
    function.set_description(args.description);
    function.set_wrap(args.wrap);
    function.set_condition(args.condition);
    function.set_shell(args.shell);

    let hooks = Hooks::new(args.pre_hook, args.post_hook);
    function.set_hooks(Some(hooks));

    functions.append(function);
    functions.save()?;

    Ok(())
}

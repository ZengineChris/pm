use clap::CommandFactory;
use clap_complete::{generate, Shell as ClapShell};
use std::io;

use crate::cli::{Cli, CompletionsArgs, Shell};
use crate::error::Result;

pub fn execute(args: CompletionsArgs) -> Result<()> {
    let mut cmd = Cli::command();
    let shell = match args.shell {
        Shell::Bash => ClapShell::Bash,
        Shell::Zsh => ClapShell::Zsh,
        Shell::Fish => ClapShell::Fish,
        Shell::PowerShell => ClapShell::PowerShell,
        Shell::Elvish => ClapShell::Elvish,
    };

    generate(shell, &mut cmd, "pm", &mut io::stdout());
    Ok(())
}

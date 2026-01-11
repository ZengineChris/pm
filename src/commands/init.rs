use crate::cli::InitArgs;
use crate::config::Config;
use crate::error::Result;
use colored::Colorize;

pub fn execute(args: InitArgs) -> Result<()> {
    Config::init(args.force)?;

    let config_path = crate::config::paths::get_config_file_path()?;

    println!(
        "{} Configuration file created at {}",
        "✓".green().bold(),
        config_path.display().to_string().cyan()
    );

    println!("\nYou can now start adding projects with:");
    println!("  {} pm create <project-name>", "$".bright_black());

    Ok(())
}

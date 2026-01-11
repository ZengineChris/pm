mod cli;
mod commands;
mod config;
mod error;
mod git;
mod models;
mod output;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => commands::init::execute(args),
        Commands::Create(ref args) => commands::create::execute(args, &cli),
        Commands::List(ref args) => commands::list::execute(args, &cli),
        Commands::Delete(ref args) => commands::delete::execute(args, &cli),
        Commands::Edit(ref args) => commands::edit::execute(args, &cli),
        Commands::Status(ref args) => commands::status::execute(args, &cli),
        Commands::Update(ref args) => commands::update::execute(args, &cli),
        Commands::Search(ref args) => commands::search::execute(args, &cli),
        Commands::Navigate(args) => commands::navigate::execute(args),
        Commands::Completions(args) => commands::completions::execute(args),
    }
}

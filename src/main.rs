use clap::Parser;

use gdcli::cli::{Cli, Commands, ExtensionCommands, GameCommands};
use gdcli::commands::{game, extension};
use gdcli::error::Result;

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    
    run(cli)?;
    Ok(())
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Game { action: GameCommands::New(args) } => game::new(args),
        Commands::Extension { action: ExtensionCommands::New(args) } => extension::new(args),
    }
}
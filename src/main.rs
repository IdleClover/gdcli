use std::env;

use clap::Parser;

use gdcli::cli::{Cli, Commands, ExtensionCommands, GameCommands};
use gdcli::commands::{completions, extension, game, info};
use gdcli::error::Result;

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    run(cli)?;
    Ok(())
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Game {
            action: GameCommands::New(args),
        } => game::new(args),
        Commands::Extension {
            action: ExtensionCommands::New(args),
        } => extension::new(args),
        Commands::Completions { shell, install } => completions::handle(shell, install),
        Commands::Info { path } => info::inspect(
            path.ok_or("Couldn't find current dir, a path must be specified")
                .or(env::current_dir())?,
        ),
    }
}

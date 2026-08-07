use std::process::ExitCode;

use clap::Parser;

use gdcli::cli::game::GameCommands;
use gdcli::cli::{Cli, Commands, build, completions, game, gdextension, info};
use gdcli::error::{Error, Result};

fn main() -> ExitCode {
    env_logger::init();
    let cli = Cli::parse();

    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => handle_error(e),
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Game {
            action: GameCommands::New(args),
        } => game::new(args),
        Commands::Extension { action } => gdextension::run(action),
        Commands::Completions { shell, install } => completions::handle(shell, install),
        Commands::Info { path } => info::inspect(path),
        Commands::Build { path, args } => build::handle(path, args),
    }
}

fn handle_error(e: Error) -> ExitCode {
    eprintln!("{e}");
    let mut source = std::error::Error::source(&e);
    while let Some(s) = source {
        eprintln!("\tCaused by: {s}");
        source = s.source();
    }
    ExitCode::FAILURE
}

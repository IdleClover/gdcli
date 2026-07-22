use std::{env, path::PathBuf};

use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;

use crate::error::Result;

fn working_directory() -> String {
    env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| ".".into())
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(alias = "ext")]
    Extension {
        #[command(subcommand)]
        action: ExtensionCommands
    },
    Game {
        #[command(subcommand)]
        action: GameCommands
    },
    Completions {
        shell: Option<Shell>,
        #[arg(short, long)]
        install: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum ExtensionCommands {
    New(NewArgs),
}

#[derive(Subcommand, Debug)]
pub enum GameCommands {
    New(NewArgs),
}

#[derive(Args, Debug)]
pub struct NewArgs {
    pub name: String,
    
    #[arg(default_value_t = working_directory())]
    pub path: String,

    #[arg(short, long)]
    pub version: Option<String>,

    #[arg(short, long, default_value="https://github.com/IdleClover/template-gdext.git")]
    pub template: String,
}

impl NewArgs {
    pub fn get_path(&self) -> Result<PathBuf> {
        let path = PathBuf::from(self.path.clone());
        let path = PathBuf::from(working_directory()).join(path);

        if !path.exists() {
            return Err(format!("`{}` don't exist", self.path).into());
        }

        if !path.is_dir() {
            return Err(format!("`{}` is not a folder", self.path).into());
        }

        let path = path.join(self.name.clone());
        if path.exists() {
            return Err(format!("`{}` already exists", path.display()).into());
        }

        Ok(path)
    }
}
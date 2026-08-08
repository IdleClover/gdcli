pub mod build;
pub mod completions;
pub mod game;
pub mod gdextension;
pub mod info;
pub mod package;

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;

use crate::{
    cli::{
        build::BuildArgs, game::GameCommands, gdextension::GdextensionCommands,
        package::PackageCommand,
    },
    error::project::{NewError, ProjectError},
    working_directory,
};

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
        action: GdextensionCommands,
    },
    Game {
        #[command(subcommand)]
        action: GameCommands,
    },
    #[command(alias = "pkg")]
    Package {
        #[command(subcommand)]
        action: PackageCommand,
    },
    Completions {
        shell: Option<Shell>,

        #[arg(short, long)]
        install: bool,
    },
    Info {
        path: Option<PathBuf>,
    },
    Build {
        path: Option<PathBuf>,
        #[command(flatten)]
        args: BuildArgs,
    },
}

#[derive(Args, Debug)]
pub struct NewArgs {
    pub name: String,

    #[arg(default_value_t = working_directory())]
    pub path: String,

    #[arg(short, long)]
    pub branch: Option<String>,

    #[arg(
        short,
        long,
        default_value = "https://github.com/IdleClover/template-gdext.git"
    )]
    pub template: String,
}

impl NewArgs {
    pub fn get_path(&self) -> Result<PathBuf, ProjectError> {
        let path = PathBuf::from(self.path.clone());
        let path = PathBuf::from(working_directory()).join(path);

        if !path.exists() {
            return Err(NewError::PathNotFound(path).into());
        }

        if !path.is_dir() {
            return Err(NewError::NotADirectory(path).into());
        }

        let path = path.join(self.name.clone());
        if path.exists() {
            return Err(NewError::AlreadyExists(path).into());
        }

        Ok(path)
    }
}

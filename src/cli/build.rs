use clap::Parser;
use std::path::PathBuf;

use crate::{
    error::Result,
    project::{HasProject, file::ProjectFile},
};

#[derive(Parser, Debug)]
pub struct BuildArgs {
    #[arg(short, long, default_value_t = false)]
    pub release: bool,

    #[arg(
        long = "compile-command",
        default_value_t = false,
        help = "Extension only"
    )]
    pub extension_compile_command: bool,

    #[arg(long = "preset", help = "Game only")]
    pub game_export_preset: Option<String>,
}

pub fn handle(path: Option<PathBuf>, args: BuildArgs) -> Result<()> {
    let project = ProjectFile::try_from(path)?;
    project.build(args)
}

use crate::{cli::NewArgs, error::Result};
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum GameCommands {
    New(NewArgs),
}

pub fn new(_args: NewArgs) -> Result<()> {
    unimplemented!()
}

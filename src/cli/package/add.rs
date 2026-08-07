//! Add a package to a project.

use clap::Parser;

use crate::error::Result;

#[derive(Parser, Debug)]
pub struct AddArgs {
    pub name: String,
}

pub fn add(_args: AddArgs) -> Result<()> {
    Ok(())
}

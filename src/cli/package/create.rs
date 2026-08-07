use clap::Parser;

use crate::error::Result;

#[derive(Parser, Debug)]
pub struct CreateArgs {
    /// Name of the package
    pub name: String,
    /// Path of the package
    pub path: Option<String>,
}

pub fn create(_args: CreateArgs) -> Result<()> {
    Ok(())
}

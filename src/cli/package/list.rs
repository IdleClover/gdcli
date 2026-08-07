//! List all the packages used inside a project.

use clap::Parser;

use crate::error::Result;

#[derive(Parser, Debug)]
pub struct ListArgs {}

/// List all the packages used inside a project.
pub fn list(_args: ListArgs) -> Result<()> {
    Ok(())
}

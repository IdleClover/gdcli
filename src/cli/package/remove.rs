use clap::Parser;

use crate::error::Result;

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    pub name: String,
}

pub fn remove(_args: RemoveArgs) -> Result<()> {
    Ok(())
}

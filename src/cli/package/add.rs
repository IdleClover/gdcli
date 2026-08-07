use clap::Parser;

use crate::error::Result;

#[derive(Parser, Debug)]
pub struct AddArgs {
    pub name: String,
}

pub fn add(args: AddArgs) -> Result<()> {
    Ok(())
}

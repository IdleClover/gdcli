use clap::Parser;

use crate::error::Result;

#[derive(Parser, Debug)]
pub struct UpdateArgs {
    pub name: Option<String>,
}

pub fn update(_args: UpdateArgs) -> Result<()> {
    Ok(())
}

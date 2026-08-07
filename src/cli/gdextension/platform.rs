use clap::{Args, ValueEnum};

use crate::error::Result;

pub fn run(args: PlatformArgs) -> Result<()> {
    match args.action {
        Action::Add => Ok(()),
        Action::Remove => Ok(()),
    }
}

#[derive(Args, Debug)]
pub struct PlatformArgs {
    #[arg(value_enum)]
    pub action: Action,
    #[arg(value_enum)]
    pub name: Platform,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Action {
    Add,
    Remove,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Platform {
    Linux,
    Macos,
    Windows,
    Android,
    Ios,
    Web,
}

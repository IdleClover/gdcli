pub mod add;
pub mod remove;
pub mod update;

use clap::Subcommand;

use crate::{
    cli::package::{add::AddArgs, remove::RemoveArgs, update::UpdateArgs},
    error::Result,
};

pub fn run(action: PackageCommand) -> Result<()> {
    match action {
        PackageCommand::Add(args) => add::add(args),
        PackageCommand::Remove(args) => remove::remove(args),
        PackageCommand::Update(args) => update::update(args),
    }
}

#[derive(Subcommand, Debug)]
pub enum PackageCommand {
    Add(AddArgs),
    Remove(RemoveArgs),
    Update(UpdateArgs),
}

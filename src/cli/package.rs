//! Godot package. A package can be a plugin, a gdextension addon,
//! or simply content that can be shared between games.

pub mod add;
pub mod create;
pub mod list;
pub mod remove;
pub mod update;

use clap::Subcommand;

use crate::{
    cli::package::{
        add::AddArgs, create::CreateArgs, list::ListArgs, remove::RemoveArgs, update::UpdateArgs,
    },
    error::Result,
};

pub fn run(action: PackageCommand) -> Result<()> {
    match action {
        PackageCommand::Add(args) => add::add(args),
        PackageCommand::Remove(args) => remove::remove(args),
        PackageCommand::Update(args) => update::update(args),
        PackageCommand::Create(args) => create::create(args),
        PackageCommand::List(args) => list::list(args),
    }
}

#[derive(Subcommand, Debug)]
pub enum PackageCommand {
    Add(AddArgs),
    Remove(RemoveArgs),
    Update(UpdateArgs),
    Create(CreateArgs),
    List(ListArgs),
}

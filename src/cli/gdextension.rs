pub mod platform;

use crate::{
    cli::{NewArgs, gdextension::platform::PlatformArgs},
    error::Result,
    package::Package,
    project::{
        HasProject, ProjectLike,
        gdextension::{self, GdextensionTarget},
    },
};
use clap::{Args, Parser, Subcommand};

pub fn run(action: GdextensionCommands) -> Result<()> {
    match action {
        GdextensionCommands::New(args) => new(args),
        GdextensionCommands::Platform(args) => platform::run(args),
    }
}

#[derive(Subcommand, Debug)]
pub enum GdextensionCommands {
    New(GdextensionNewArgs),
    Platform(PlatformArgs),
}

#[derive(Parser, Debug)]
pub struct GdextensionNewArgs {
    #[command(flatten)]
    pub common: NewArgs,

    #[command(flatten)]
    pub target: TargetArgs,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct TargetArgs {
    /// Editor only extension
    #[arg(short, long)]
    pub editor: bool,
    /// Game only extension
    #[arg(short, long)]
    pub runtime: bool,
    /// Both editor and game extension
    #[arg(short, long)]
    pub both: bool,
}

impl TargetArgs {
    pub fn resolve(&self) -> GdextensionTarget {
        if self.editor {
            GdextensionTarget::Editor
        } else if self.both {
            GdextensionTarget::Both
        } else {
            // Default
            GdextensionTarget::Runtime
        }
    }
}

pub fn new(args: GdextensionNewArgs) -> Result<()> {
    let name = args.common.name.clone();
    let path = args.common.get_path()?;

    let project = gdextension::create(
        &args.common.template,
        &path,
        name.clone(),
        args.common.version.as_deref(),
        args.target.resolve(),
    )?;
    project.post_installation()?;
    project.save()?;

    let pkg = Package::new(name, &path);
    pkg.save()?;

    project.commit_all("Setup extension project")?;
    Ok(())
}

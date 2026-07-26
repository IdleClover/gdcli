use crate::{
    cli::NewArgs,
    error::Result,
    project::{
        HasProject, ProjectLike,
        gdext::{self, GdextTarget},
    },
};
use clap::{Args, Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum ExtensionCommands {
    New(ExtensionNewArgs),
}

#[derive(Parser, Debug)]
pub struct ExtensionNewArgs {
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
    pub fn resolve(&self) -> GdextTarget {
        if self.editor {
            GdextTarget::Editor
        } else if self.both {
            GdextTarget::Both
        } else {
            // Default
            GdextTarget::Runtime
        }
    }
}

pub fn new(args: ExtensionNewArgs) -> Result<()> {
    let project = gdext::create(
        &args.common.template,
        &args.common.get_path()?,
        args.common.name,
        args.common.version.as_deref(),
        args.target.resolve(),
    )?;
    project.post_installation()?;
    project.save()?;
    project.commit_all("Setup extension project")?;

    Ok(())
}

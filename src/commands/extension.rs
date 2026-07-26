use crate::{
    cli::NewArgs,
    error::Result,
    project::{HasProject, gdext},
};

pub fn new(args: NewArgs) -> Result<()> {
    let project = gdext::create(
        &args.template,
        &args.get_path()?,
        args.name,
        args.version.as_deref(),
    )?;
    project.save()?;
    project.commit_all("Setup extension project")?;

    Ok(())
}

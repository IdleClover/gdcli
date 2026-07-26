use crate::{cli::NewArgs, error::Result, project::gdext};

pub fn new(args: NewArgs) -> Result<()> {
    gdext::create(
        &args.template,
        &args.get_path()?,
        args.name,
        args.version.as_deref(),
    )?
    .save()?;

    Ok(())
}

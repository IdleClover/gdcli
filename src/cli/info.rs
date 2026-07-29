use std::path::PathBuf;

use crate::{
    error::Result,
    project::{ProjectLike, file::ProjectFile},
};

pub fn inspect(path: Option<PathBuf>) -> Result<()> {
    let project = ProjectFile::try_from(path)?;
    println!("{}", project.name());
    Ok(())
}

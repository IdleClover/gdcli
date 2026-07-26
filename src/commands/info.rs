use std::path::PathBuf;

use crate::{
    error::Result,
    project::{HasProject, file::ProjectFile},
};

pub fn inspect(path: PathBuf) -> Result<()> {
    let project = ProjectFile::try_from(path)?;
    println!("{}", project.name());
    Ok(())
}

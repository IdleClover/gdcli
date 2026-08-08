use std::path::PathBuf;

use clap::Parser;

use crate::{
    error::package::{NewError, PackageError},
    package::Package,
    working_directory,
};

#[derive(Parser, Debug)]
pub struct InitArgs {
    /// Name of the package
    pub name: String,
    /// Path of the package
    #[arg(default_value_t = working_directory())]
    pub path: String,
}

pub fn init(args: InitArgs) -> crate::error::Result<()> {
    let path = args.path().map_err(PackageError::from)?;
    let pkg = Package::create(args.name, &path).map_err(PackageError::from)?;
    pkg.save()?;
    println!("Package named '{}' initialized", pkg);
    Ok(())
}

impl InitArgs {
    pub fn path(&self) -> Result<PathBuf, NewError> {
        let path = PathBuf::from(self.path.clone());
        let path = PathBuf::from(working_directory()).join(path);

        if !path.exists() {
            return Err(NewError::PathNotFound(path).into());
        }

        if !path.is_dir() {
            return Err(NewError::NotADirectory(path).into());
        }

        Ok(path)
    }
}

use std::{
    fs,
    path::{Path, PathBuf},
};

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::error::{Result, fs::FsError, toml::TomlError};

const PACKAGE_FILENAME: &str = "package.gdcli";

#[derive(Serialize, Deserialize)]
pub struct Package {
    #[serde(skip)]
    pub path: PathBuf,
    pub name: String,
    pub version: Version,
}

impl Package {
    pub fn new(name: String, folder: &Path) -> Self {
        Self {
            name,
            path: folder.join(PACKAGE_FILENAME),
            version: Version::new(0, 1, 0),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn save(&self) -> Result<()> {
        let toml = toml::to_string(&self).map_err(TomlError::SerializationFailed)?;
        fs::write(self.path(), toml).map_err(|e| FsError::WriteFailed {
            path: self.path().to_path_buf(),
            source: e,
        })?;
        Ok(())
    }
}

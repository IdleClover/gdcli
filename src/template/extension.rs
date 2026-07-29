use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{Result, fs::FsError, toml::TomlError};

const TEMPLATE_FILENAME: &str = "template.toml";

#[derive(Serialize, Deserialize)]
pub struct ExtensionTemplate {
    #[serde(skip)]
    pub path: PathBuf,
    pub gdextension_path: PathBuf,
}

impl ExtensionTemplate {
    pub fn open(mut path: PathBuf) -> Result<Self> {
        if path.is_dir() {
            path.push(TEMPLATE_FILENAME);
        }
        let content = fs::read_to_string(&path).map_err(|e| FsError::ReadFailed {
            path: path.clone(),
            source: e,
        })?;
        let mut template: ExtensionTemplate =
            toml::from_str(&content).map_err(TomlError::DeserializationFailed)?;

        template.path = path;

        Ok(template)
    }

    pub fn save(&self) -> Result<()> {
        let content = toml::to_string(self).map_err(TomlError::SerializationFailed)?;
        fs::write(&self.path, content).map_err(|e| FsError::WriteFailed {
            path: self.path.clone(),
            source: e,
        })?;
        Ok(())
    }

    pub fn rename_gdextension_file(&mut self, name: &str) -> Result<()> {
        let old = self.gdextension_path();
        self.gdextension_path = self
            .gdextension_path
            .parent()
            .unwrap()
            .to_path_buf()
            .join(format!("{name}.gdextension"));
        let new = self.gdextension_path();

        fs::rename(&old, &new).map_err(|e| FsError::RenameFailed {
            from: old.clone(),
            to: new.clone(),
            source: e,
        })?;
        Ok(())
    }

    pub fn gdextension_path(&self) -> PathBuf {
        self.path.parent().unwrap().join(&self.gdextension_path)
    }
}

use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::Result;

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
        let content = fs::read_to_string(&path)?;
        let mut template: ExtensionTemplate = toml::from_str(&content)
            .map_err(|e| format!("Couldn't load ExtensionTemplate from str: {e}"))?;

        template.path = path;

        Ok(template)
    }

    pub fn save(&self) -> Result<()> {
        let content = toml::to_string(self)?;
        fs::write(&self.path, content).map_err(|e| {
            format!(
                "Couldn't save ExtensionTemplate in {}: {e}",
                self.path.display()
            )
            .into()
        })
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

        fs::rename(&old, &new).map_err(|e| {
            format!(
                "Failed to rename {} file to {}: {e}",
                old.display(),
                new.display()
            )
            .into()
        })
    }

    pub fn gdextension_path(&self) -> PathBuf {
        self.path.parent().unwrap().join(&self.gdextension_path)
    }
}

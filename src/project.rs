use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::error::Result;

pub mod game;
pub mod gdext;

#[derive(Serialize, Deserialize)]
pub struct Project {
    #[serde(skip)]
    pub path: PathBuf,
    pub name: String,
}

impl Project {
    pub fn new(name: String, folder: &Path) -> Self {
        Self {
            name: name,
            path: folder.join("project.gdcli"),
        }
    }
}

pub trait HasProject {
    fn base(&self) -> &Project;
    fn base_mut(&mut self) -> &mut Project;

    fn path(&self) -> &PathBuf {
        &self.base().path
    }

    fn name(&self) -> &str {
        &self.base().name
    }

    fn post_installation(&self) -> Result<()> {
        Ok(())
    }

    fn save(&self) -> Result<()>
    where
        Self: Serialize + DeserializeOwned,
    {
        let toml = toml::to_string(&self)?;
        let mut file = File::create(self.path())?;
        file.write_all(toml.as_bytes())?;
        Ok(())
    }
}

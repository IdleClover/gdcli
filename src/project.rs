use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::Result;

pub mod file;
pub mod game;
pub mod gdext;

#[derive(Serialize, Deserialize)]
pub struct Project {
    #[serde(skip)]
    pub path: PathBuf,
    pub name: String,
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
}

impl Project {
    pub fn new(name: String, folder: &Path) -> Self {
        Self {
            name: name,
            path: folder.join("project.gdcli"),
        }
    }
}

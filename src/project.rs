use std::path::{Path, PathBuf};

use git2::{Oid, Repository};
use serde::{Deserialize, Serialize};

use crate::{error::Result, git::GdCliRepository};

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

    fn repository(&self) -> Result<Repository> {
        self.base().repository()
    }

    fn commit_all(&self, message: &str) -> Result<Oid> {
        self.base().commit_all(message)
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

    pub fn repository(&self) -> Result<Repository> {
        Repository::open(&self.path.parent().ok_or("Parent not found")?)
            .map_err(|e| format!("Failed to open {} repository: {e}", self.path.display()).into())
    }

    pub fn commit_all(&self, message: &str) -> Result<Oid> {
        self.repository()?.commit_all(message)
    }
}

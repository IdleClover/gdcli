pub mod file;
pub mod game;
pub mod gdext;

use std::path::{Path, PathBuf};

use git2::{Oid, Repository};
use serde::{Deserialize, Serialize};

use crate::{error::Result, git::GdCliRepository};

const PROJECT_FILENAME: &str = "project.gdcli";

#[derive(Serialize, Deserialize)]
pub struct Project {
    #[serde(skip)]
    pub path: PathBuf,
    pub name: String,
}

pub trait ProjectLike {
    fn path(&self) -> &PathBuf;
    fn name(&self) -> &str;

    fn repository(&self) -> Result<Repository> {
        let path = self.path();
        Repository::open(&path.parent().ok_or("Parent not found")?)
            .map_err(|e| format!("Failed to open {} repository: {e}", path.display()).into())
    }

    fn commit_all(&self, message: &str) -> Result<Oid> {
        self.repository()?.commit_all(message)
    }
}

pub trait HasProject {
    fn base(&self) -> &Project;
    fn base_mut(&mut self) -> &mut Project;

    fn post_installation(&self) -> Result<()> {
        Ok(())
    }
}

impl Project {
    pub fn new(name: String, folder: &Path) -> Self {
        Self {
            name: name,
            path: folder.join(PROJECT_FILENAME),
        }
    }
}

impl ProjectLike for Project {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl<T: HasProject> ProjectLike for T {
    fn path(&self) -> &PathBuf {
        self.base().path()
    }

    fn name(&self) -> &str {
        self.base().name()
    }
}

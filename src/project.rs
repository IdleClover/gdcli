pub mod file;
pub mod game;
pub mod gdext;

use std::path::{Path, PathBuf};

use git2::{Oid, Repository};
use serde::{Deserialize, Serialize};

use crate::{
    cli::build::BuildArgs,
    error::{Result, git::GitError, path::PathError},
    git::GdCliRepository,
};

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

    fn dir(&self) -> &Path {
        self.path().parent().unwrap()
    }

    fn repository(&self) -> Result<Repository> {
        let path = self.path();
        let repository = Repository::open(path.parent().ok_or(PathError::NoParent(path.clone()))?)
            .map_err(GitError::from)?;
        Ok(repository)
    }

    fn commit_all(&self, message: &str) -> Result<Oid> {
        let oid = self.repository()?.commit_all(message)?;
        Ok(oid)
    }
}

pub trait HasProject {
    fn base(&self) -> &Project;
    fn base_mut(&mut self) -> &mut Project;

    fn post_installation(&self) -> Result<()> {
        Ok(())
    }

    fn build(&self, args: BuildArgs) -> Result<()>;
}

impl Project {
    pub fn new(name: String, folder: &Path) -> Self {
        Self {
            name,
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

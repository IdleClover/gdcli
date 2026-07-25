use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{
    error::Result,
    git,
    project::{HasProject, Project},
    ui::RepositoryProgressBar,
};

#[derive(Serialize, Deserialize)]
pub struct GdextProject {
    #[serde(rename(serialize = "project", deserialize = "base"))]
    pub base: Project,
    pub target: GdextTarget,
}

#[derive(Serialize, Deserialize)]
pub enum GdextTarget {
    Editor,
    Runtime,
    Both,
}

impl GdextProject {
    fn new(name: String, folder: &Path) -> Self {
        GdextProject {
            base: Project::new(name, folder),
            target: GdextTarget::Editor,
        }
    }
}

impl HasProject for GdextProject {
    fn base(&self) -> &Project {
        &self.base
    }

    fn base_mut(&mut self) -> &mut Project {
        &mut self.base
    }
}

pub fn create(url: &str, dest: &Path, name: String, version: Option<&str>) -> Result<GdextProject> {
    git::clone(
        url,
        dest,
        version,
        &[("EXTENSION-NAME", &name)],
        RepositoryProgressBar::new(url.to_string()),
    )?;

    Ok(GdextProject::new(name, dest))
}

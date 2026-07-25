use std::path::Path;

use crate::{
    error::Result,
    git,
    project::{HasProject, Project},
    ui::RepositoryProgressBar,
};

pub struct GdextProject {
    pub base: Project,
    pub target: GdextTarget,
}

impl GdextProject {
    fn new(name: String) -> Self {
        GdextProject {
            base: Project { name: name },
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

pub enum GdextTarget {
    Editor,
    Runtime,
    Both,
}

pub fn create(url: &str, dest: &Path, name: String, version: Option<&str>) -> Result<GdextProject> {
    git::clone(
        url,
        dest,
        version,
        &[("EXTENSION-NAME", &name)],
        RepositoryProgressBar::new(url.to_string()),
    )?;

    Ok(GdextProject::new(name))
}

use std::path::Path;

use indicatif::ProgressStyle;

use crate::{error::Result, git, project::{HasProject, Project}};

pub struct GdextProject {
    pub base: Project,
    pub target: GdextTarget,
}

impl GdextProject {
    fn new(name: String) -> Self {
        GdextProject {
            base: Project { name: name },
            target: GdextTarget::Editor
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
    Both
}

pub fn create(url: &str, dest: &Path, name: String, version: Option<&str>) -> Result<GdextProject> {
    let pb = indicatif::ProgressBar::new(1);
    pb.set_style(
        ProgressStyle::with_template(
            "{msg} [{bar:40.cyan/blue}] {pos}/{len} objets ({eta})",
        )
        .unwrap()
        .progress_chars("=>-"),
    );
    pb.set_message("Cloning template");

    git::clone(
        url,
        dest,
        version,
        &[("EXTENSION-NAME", &name),],
        &pb
    )?;

    Ok(GdextProject::new(name))
}
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{
    error::Result,
    git,
    project::{HasProject, Project, ProjectLike, file::ProjectFile},
    template::{extension::ExtensionTemplate, replace_in_files},
    ui::RepositoryProgressBar,
};

#[derive(Serialize, Deserialize)]
pub struct GdextProject {
    #[serde(rename = "project")]
    pub base: Project,
    pub target: GdextTarget,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum GdextTarget {
    Editor,
    #[default]
    Runtime,
    Both,
}

impl GdextProject {
    fn new(name: String, folder: &Path, target: GdextTarget) -> Self {
        GdextProject {
            base: Project::new(name, folder),
            target,
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

    fn post_installation(&self) -> Result<()> {
        replace_in_files(&self.repository()?, &[("EXTENSION-NAME", self.name())])?;

        let mut template = ExtensionTemplate::open(self.dir().to_path_buf())?;
        template.rename_gdextension_file(self.name())?;
        template.save()
    }

    fn build(&self, args: crate::cli::build::BuildArgs) -> Result<()> {
        println!("cc: {}", args.extension_compile_command);
        Ok(())
    }
}

pub fn create(
    url: &str,
    dest: &Path,
    name: String,
    version: Option<&str>,
    target: GdextTarget,
) -> Result<ProjectFile> {
    git::clone(
        url,
        dest,
        version,
        RepositoryProgressBar::new(url.to_string()),
    )?;
    Ok(GdextProject::new(name, dest, target).into())
}

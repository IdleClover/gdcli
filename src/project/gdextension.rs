use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{
    error::Result,
    git,
    project::{HasProject, Project, ProjectLike, file::ProjectFile},
    template::{extension::GdextensionTemplate, replace_in_files},
    ui::RepositoryProgressBar,
};

#[derive(Serialize, Deserialize)]
pub struct GdextensionProject {
    #[serde(rename = "project")]
    pub base: Project,
    pub target: GdextensionTarget,
    pub platforms: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum GdextensionTarget {
    Editor,
    #[default]
    Runtime,
    Both,
}

impl GdextensionProject {
    fn new(name: String, folder: &Path, target: GdextensionTarget) -> Self {
        GdextensionProject {
            base: Project::new(name, folder),
            target,
            platforms: vec!["linux".into()],
        }
    }
}

impl HasProject for GdextensionProject {
    fn base(&self) -> &Project {
        &self.base
    }

    fn base_mut(&mut self) -> &mut Project {
        &mut self.base
    }

    fn post_installation(&self) -> Result<()> {
        replace_in_files(&self.repository()?, &[("EXTENSION-NAME", self.name())])?;

        let mut template = GdextensionTemplate::open(self.dir().to_path_buf())?;
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
    target: GdextensionTarget,
) -> Result<ProjectFile> {
    git::clone(
        url,
        dest,
        version,
        RepositoryProgressBar::new(url.to_string()),
    )?;
    Ok(GdextensionProject::new(name, dest, target).into())
}

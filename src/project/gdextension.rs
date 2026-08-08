use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{
    error::Result,
    git,
    project::{HasProject, Project, ProjectLike, file::ProjectFile},
    template::{gdextension::GdextensionTemplate, replace_in_files},
    ui::RepositoryProgressBar,
};

#[derive(Serialize, Deserialize)]
pub struct GdextensionProject {
    pub target: GdextensionTarget,
    pub platforms: Vec<String>,

    #[serde(rename = "project")]
    pub base: Project,
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
            target,
            platforms: vec!["linux".into()],
            base: Project::new(name, folder),
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
    branch: Option<&str>,
    target: GdextensionTarget,
) -> Result<ProjectFile> {
    git::clone(
        url,
        dest,
        branch,
        RepositoryProgressBar::new(url.to_string()),
    )?;
    Ok(GdextensionProject::new(name, dest, target).into())
}

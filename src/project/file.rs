use std::{error::Error, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    error::Result,
    project::{HasProject, Project, game::GameProject, gdext::GdextProject},
};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProjectFile {
    Game(GameProject),
    Gdext(GdextProject),
}

impl ProjectFile {
    pub fn save(&self) -> Result<()> {
        let toml = toml::to_string(&self)?;
        fs::write(self.path(), toml)?;
        Ok(())
    }
}

impl HasProject for ProjectFile {
    fn base(&self) -> &Project {
        match self {
            ProjectFile::Game(p) => p.base(),
            ProjectFile::Gdext(p) => p.base(),
        }
    }

    fn base_mut(&mut self) -> &mut Project {
        match self {
            ProjectFile::Game(p) => p.base_mut(),
            ProjectFile::Gdext(p) => p.base_mut(),
        }
    }
}

impl TryFrom<PathBuf> for ProjectFile {
    type Error = Box<dyn Error>;

    fn try_from(mut path: PathBuf) -> Result<Self> {
        if path.is_dir() {
            path.push("project.gdcli");
        }
        let content = fs::read_to_string(&path)?;
        let mut project: ProjectFile = toml::from_str(&content)?;
        project.base_mut().path = path;
        Ok(project)
    }
}

impl From<GameProject> for ProjectFile {
    fn from(value: GameProject) -> Self {
        Self::Game(value)
    }
}

impl From<GdextProject> for ProjectFile {
    fn from(value: GdextProject) -> Self {
        Self::Gdext(value)
    }
}

use std::{env, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    cli::build::BuildArgs,
    error::{Error, Result, fs::FsError, toml::TomlError},
    project::{
        HasProject, PROJECT_FILENAME, Project, ProjectLike, game::GameProject,
        gdextension::GdextensionProject,
    },
};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProjectFile {
    Game(GameProject),
    Gdextension(GdextensionProject),
}

impl ProjectFile {
    pub fn save(&self) -> Result<()> {
        let toml = toml::to_string(&self).map_err(TomlError::SerializationFailed)?;
        fs::write(self.path(), toml).map_err(|e| FsError::WriteFailed {
            path: self.path().clone(),
            source: e,
        })?;
        Ok(())
    }
}

macro_rules! dispatch {
    ($self:ident.$method:ident($($arg:expr),*)) => {
        match $self {
            ProjectFile::Game(p) => p.$method($($arg),*),
            ProjectFile::Gdextension(p) => p.$method($($arg),*),
        }
    };
}

impl HasProject for ProjectFile {
    fn base(&self) -> &Project {
        dispatch!(self.base())
    }

    fn base_mut(&mut self) -> &mut Project {
        dispatch!(self.base_mut())
    }

    fn post_installation(&self) -> Result<()> {
        dispatch!(self.post_installation())
    }

    fn build(&self, args: BuildArgs) -> Result<()> {
        dispatch!(self.build(args))
    }
}

impl TryFrom<PathBuf> for ProjectFile {
    type Error = Error;

    fn try_from(mut path: PathBuf) -> Result<Self> {
        if path.is_dir() {
            path.push(PROJECT_FILENAME);
        }
        let content = fs::read_to_string(&path).map_err(|e| FsError::ReadFailed {
            path: path.clone(),
            source: e,
        })?;
        let mut project: ProjectFile =
            toml::from_str(&content).map_err(TomlError::DeserializationFailed)?;
        project.base_mut().path = path;
        Ok(project)
    }
}

impl TryFrom<Option<PathBuf>> for ProjectFile {
    type Error = Error;

    fn try_from(value: Option<PathBuf>) -> Result<Self> {
        let path = value.unwrap_or(env::current_dir().map_err(FsError::CurrentDirUnavailable)?);
        ProjectFile::try_from(path)
    }
}

impl From<GameProject> for ProjectFile {
    fn from(value: GameProject) -> Self {
        Self::Game(value)
    }
}

impl From<GdextensionProject> for ProjectFile {
    fn from(value: GdextensionProject) -> Self {
        Self::Gdextension(value)
    }
}

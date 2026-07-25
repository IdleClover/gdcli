use serde::{Deserialize, Serialize};

use crate::project::{HasProject, Project};

#[derive(Serialize, Deserialize)]
pub struct GameProject {
    #[serde(rename(serialize = "project", deserialize = "base"))]
    pub base: Project,
}

impl HasProject for GameProject {
    fn base(&self) -> &Project {
        &self.base
    }

    fn base_mut(&mut self) -> &mut Project {
        &mut self.base
    }
}

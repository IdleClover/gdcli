use crate::project::{HasProject, Project};

pub struct GameProject {
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
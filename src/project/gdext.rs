use crate::project::{HasProject, Project};

pub struct GdextProject {
    pub base: Project,
    pub target: GdextTarget,
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
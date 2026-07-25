use crate::error::Result;

pub mod game;
pub mod gdext;

pub struct Project {
    pub name: String,
}

pub trait HasProject {
    fn base(&self) -> &Project;
    fn base_mut(&mut self) -> &mut Project;

    fn name(&self) -> &str {
        &self.base().name
    }

    fn post_installation(&self) -> Result<()> {
        Ok(())
    }
}

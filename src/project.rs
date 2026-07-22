use crate::error::Result;

pub mod gdext;
pub mod game;

pub struct Project {
    pub name: String,
    pub version: String,
}

pub trait HasProject {
    fn base(&self) -> &Project;
    fn base_mut(&mut self) -> &mut Project;

    fn name(&self) -> &str {
        &self.base().name
    }

    fn version(&self) -> &str {
        &self.base().version
    }

    fn post_installation(&self) -> Result<()> {
        Ok(())
    }
}
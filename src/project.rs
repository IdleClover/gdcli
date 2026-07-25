use indicatif::ProgressBar;

use crate::{error::Result, git::CloneProgress};

pub mod gdext;
pub mod game;

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

impl CloneProgress for ProgressBar {
    fn on_transfer(&self, stats: git2::Progress) {
        self.set_length(stats.total_objects() as u64);
        self.set_position(stats.received_objects() as u64);
    }
    
    fn finish(&self) {
        self.finish_with_message("Template cloned");
    }
}
use git2::Submodule;
use indicatif::{ProgressBar, ProgressStyle};

use crate::git::CloneProgress;

pub struct RepositoryProgressBar(ProgressBar, String);
pub struct SubmoduleProgressBar(ProgressBar, String);

impl RepositoryProgressBar {
    pub fn new(url: String) -> Self {
        let pb = progress_bar();
        pb.set_message(format!("Cloning {} repository", url));

        RepositoryProgressBar(pb, url)
    }

    pub fn new_submodule_progress_bar(&self, submodule: &Submodule) -> SubmoduleProgressBar {
        let name = submodule.name().unwrap_or("''").to_string();
        let pb = progress_bar();
        pb.set_message(format!("Initializing {} submodule", name));

        SubmoduleProgressBar(pb, name)
    }
}

impl CloneProgress for RepositoryProgressBar {
    fn on_transfer(&self, stats: git2::Progress) {
        self.0.set_length(stats.total_objects() as u64);
        self.0.set_position(stats.received_objects() as u64);
    }

    fn finish(&self) {
        self.0
            .finish_with_message(format!("Repository {} cloned", self.1));
    }
}

impl CloneProgress for SubmoduleProgressBar {
    fn on_transfer(&self, stats: git2::Progress) {
        self.0.set_length(stats.total_objects() as u64);
        self.0.set_position(stats.received_objects() as u64);
    }

    fn finish(&self) {
        self.0
            .finish_with_message(format!("Submodule {} initialized", self.1));
    }
}

pub fn progress_bar() -> ProgressBar {
    let pb = indicatif::ProgressBar::new(1);
    pb.set_style(
        ProgressStyle::with_template("{msg} [{bar:40.cyan/blue}] {pos}/{len} objects ({eta})")
            .unwrap()
            .progress_chars("=> "),
    );
    pb
}

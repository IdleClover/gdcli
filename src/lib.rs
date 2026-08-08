use std::env;

pub mod cli;
pub mod error;
pub mod git;
pub mod package;
pub mod project;
pub mod template;
pub mod ui;
pub mod url;

pub fn working_directory() -> String {
    env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| ".".into())
}

pub mod extension;

use std::{fs, path::PathBuf};

use git2::Repository;

use crate::error::Result;

pub fn replace_in_files(repository: &Repository, replacements: &[(&str, &str)]) -> Result<()> {
    let root = repository
        .workdir()
        .expect("A cloned repository cannot be bare, wtf");

    let files: Vec<PathBuf> = repository
        .index()?
        .iter()
        .filter_map(|e| String::from_utf8(e.path).ok())
        .map(|p| root.join(PathBuf::from(p)))
        .filter(|p| p.is_file())
        .collect();

    log::info!("{} files to read", files.len());
    for path in &files {
        log::debug!("Reading {}", path.display());
        // Skip .git directory
        if path.components().any(|c| c.as_os_str() == ".git") {
            continue;
        }

        let mut content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => continue, // Skip binariy/image files
        };
        let original = content.clone();

        for (from, to) in replacements {
            content = content.replace(from, to);
        }

        if content != original {
            fs::write(path, content)?;
            log::info!("{} modified", path.display());
        }
    }

    Ok(())
}

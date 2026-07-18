use std::{error::Error, fs, path::Path};

use git2::{FetchOptions, RemoteCallbacks, Repository, build::RepoBuilder};
use walkdir::WalkDir;

use crate::error::Result;

const DEPTH: i32 = 1;

pub fn clone_repository(
    url: &String,
    into: &Path,
    callbacks: RemoteCallbacks,
    branch: &Option<String>,
    replacements: &[(&str, &str)],
) -> Result<()> {
    let repository = create_builder(callbacks, branch).clone(url, into)?;
    init_submodules(&repository)?;
    replace_in_files(repository.workdir().ok_or("The repository can't be bare")?, replacements)
}

fn replace_in_files(
    workdir: &Path,
    replacements: &[(&str, &str)],
) -> Result<()> {
    let files = WalkDir::new(workdir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file());
    for entry in files {
        let path = entry.path();
        
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

fn init_submodules(repository: &Repository) -> Result<()> {
    for mut submodule in repository.submodules()? {
        submodule.update(true, None).map_err(|e| -> Box<dyn Error> {
            let name = submodule.name().unwrap_or("missing_name");
            format!("Failed to initialize submodule {}: {}", name, e.message()).into()
        })?;
    }

    Ok(())
}

fn create_builder<'a>(callbacks: RemoteCallbacks<'a>, branch: &'a Option<String>) -> RepoBuilder<'a> {
    let mut fo = FetchOptions::new();
    fo.remote_callbacks(callbacks);
    fo.depth(DEPTH);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fo);
    if let Some(branch) = branch {
        builder.branch(&branch);
    }

    builder
}
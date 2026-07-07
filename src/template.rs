use std::{error::Error, path::Path};

use git2::{FetchOptions, RemoteCallbacks, Repository, build::RepoBuilder};

use crate::error::Result;

const DEPTH: i32 = 1;

pub fn clone_repository(url: &String, into: &Path, callbacks: RemoteCallbacks, branch: &Option<String>) -> Result<()> {
    let repository = create_builder(callbacks, branch).clone(url, into)?;
    init_submodules(&repository)
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
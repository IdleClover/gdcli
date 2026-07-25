use std::{env, error::Error, fs, path::Path};

use git2::{
    Cred, FetchOptions, Progress, RemoteCallbacks, Repository, SubmoduleUpdateOptions,
    build::RepoBuilder,
};
use walkdir::WalkDir;

use crate::{error::Result, ui::RepositoryProgressBar};

const DEPTH: i32 = 1;

enum UrlKind {
    Http,
    Ssh,
    Invalid,
}

pub trait CloneProgress {
    fn on_transfer(&self, stats: Progress);
    fn finish(&self);
}

fn classify_url(url: &str) -> UrlKind {
    if url.starts_with("http://") || url.starts_with("https://") {
        UrlKind::Http
    } else if url.starts_with("git@") || url.starts_with("ssh://") {
        UrlKind::Ssh
    } else {
        UrlKind::Invalid
    }
}

pub fn clone(
    url: &str,
    dest: &Path,
    branch: Option<&str>,
    replacements: &[(&str, &str)],
    progress: RepositoryProgressBar,
) -> Result<Repository> {
    let mut callbacks: RemoteCallbacks = match classify_url(url) {
        UrlKind::Http => http_callbacks(),
        UrlKind::Ssh => ssh_callbacks(),
        UrlKind::Invalid => return Err(format!("Invalid template url: '{}'", url).into()),
    };

    callbacks.transfer_progress(|stats| {
        progress.on_transfer(stats);
        true
    });

    let repository = create_builder(callbacks, branch).clone(url, dest)?;
    progress.finish();

    init_submodules(&repository, &progress)?;
    replace_in_files(
        repository.workdir().ok_or("The repository can't be bare")?,
        replacements,
    )?;

    Ok(repository)
}

fn http_callbacks<'a>() -> RemoteCallbacks<'a> {
    let token = env::var("GITHUB_TOKEN");

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
        if let Ok(token) = &token {
            Cred::userpass_plaintext("", &token)
        } else {
            log::warn!("An HTTP URL was provided, but the GITHUB_TOKEN environment variable is empty. If the repository is private, it cannot be cloned");
            Cred::userpass_plaintext("", "")
        }
    });

    callbacks
}

fn ssh_callbacks<'a>() -> RemoteCallbacks<'a> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
    });

    callbacks
}

fn replace_in_files(workdir: &Path, replacements: &[(&str, &str)]) -> Result<()> {
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

fn init_submodules(repository: &Repository, progress_bar: &RepositoryProgressBar) -> Result<()> {
    for mut submodule in repository.submodules()? {
        let spb = progress_bar.new_submodule_progress_bar(&submodule);

        let mut callbaks = RemoteCallbacks::new();
        callbaks.transfer_progress(|stats| {
            spb.on_transfer(stats);
            true
        });

        let mut fo = FetchOptions::new();
        fo.remote_callbacks(callbaks);

        let mut update_opts = SubmoduleUpdateOptions::new();
        update_opts.fetch(fo);

        submodule
            .update(true, Some(&mut update_opts))
            .map_err(|e| -> Box<dyn Error> {
                let name = submodule.name().unwrap_or("missing_name");
                format!("Failed to initialize submodule {}: {}", name, e.message()).into()
            })?;
        spb.finish();
    }

    Ok(())
}

fn create_builder<'a>(callbacks: RemoteCallbacks<'a>, branch: Option<&str>) -> RepoBuilder<'a> {
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

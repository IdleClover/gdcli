use std::{env, path::Path, vec};

use git2::{
    Commit, Cred, FetchOptions, IndexAddOption, Progress, RemoteCallbacks, Repository,
    SubmoduleUpdateOptions, build::RepoBuilder,
};

use crate::{
    error::git::GitError,
    ui::RepositoryProgressBar,
    url::{UrlKind, classify_url},
};

const DEPTH: i32 = 1;

pub trait CloneProgress {
    fn on_transfer(&self, stats: Progress);
    fn finish(&self);
}

pub trait GdCliRepository {
    fn commit_all(&self, message: &str) -> Result<git2::Oid, GitError>;
    fn remove_all_remotes(&self) -> Result<(), GitError>;
}

impl GdCliRepository for Repository {
    fn commit_all(&self, message: &str) -> Result<git2::Oid, GitError> {
        let mut index = self.index()?;
        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let tree_id = index.write_tree()?;
        let tree = self.find_tree(tree_id)?;

        let signature = self.signature()?;

        let parent_commit = match self.head() {
            Ok(head) => Some(head.peel_to_commit()?),
            Err(_) => None,
        };

        let parents: Vec<&Commit> = match &parent_commit {
            Some(c) => vec![c],
            None => vec![],
        };

        self.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )
        .map_err(GitError::Git2)
    }

    fn remove_all_remotes(&self) -> Result<(), GitError> {
        for remote in self.remotes()?.iter() {
            let remote = remote?;
            if let Some(name) = remote {
                self.remote_delete(name)?;
            }
        }

        Ok(())
    }
}

pub fn clone(
    url: &str,
    dest: &Path,
    branch: Option<&str>,
    progress: RepositoryProgressBar,
) -> Result<Repository, GitError> {
    let url = classify_url(url);
    let mut callbacks: RemoteCallbacks = build_callbacks(&url)?;

    callbacks.transfer_progress(|stats| {
        progress.on_transfer(stats);
        true
    });

    let repository = create_builder(callbacks, branch).clone(url.url(), dest)?;
    progress.finish();

    init_submodules(&repository, &progress)?;

    Ok(repository)
}

fn build_callbacks<'a>(url: &UrlKind) -> Result<RemoteCallbacks<'a>, GitError> {
    match url {
        UrlKind::Http(_) => Ok(http_callbacks()),
        UrlKind::Ssh(_) => Ok(ssh_callbacks()),
        UrlKind::Invalid(_) => Err(GitError::InvalidUrl(url.clone())),
    }
}

fn http_callbacks<'a>() -> RemoteCallbacks<'a> {
    let token = env::var("GITHUB_TOKEN");

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
        if let Ok(token) = &token {
            Cred::userpass_plaintext("", token)
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

fn init_submodules(
    repository: &Repository,
    progress_bar: &RepositoryProgressBar,
) -> Result<(), GitError> {
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
            .map_err(|e| GitError::SubmoduleInitializationFailed {
                name: submodule.name().unwrap_or("missing_name").into(),
                source: e,
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
        builder.branch(branch);
    }

    builder
}

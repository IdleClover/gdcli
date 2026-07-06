use std::env;

use clap::builder;
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository, build::RepoBuilder};

use crate::{cli::NewArgs, error::Result};

enum UrlKind {
    Http,
    Ssh,
    Invalid,
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

pub fn new(args: NewArgs) -> Result<()> {
    match classify_url(&args.template) {
        UrlKind::Http => new_http(args),
        UrlKind::Ssh => new_ssh(args),
        UrlKind::Invalid => Err(format!("Invalid template url: '{}'", args.template).into())
    }
}


fn new_http(args: NewArgs) -> Result<()> {
    let url = &args.template;

    let token = env::var("GITHUB_TOKEN");

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
        if let Ok(token) = &token {
            Cred::userpass_plaintext("", &token)
        } else {
            Cred::userpass_plaintext("", "")
        }
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(callbacks);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fo);

    let repository = builder.clone(url, args.get_path()?.as_ref())?;

    Ok(())
}

fn new_ssh(args: NewArgs) -> Result<()> {
    let url = &args.template;

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(callbacks);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fo);

    let repository = builder.clone(url, args.get_path()?.as_ref())?;

    Ok(())
}
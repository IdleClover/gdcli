use std::env;
use git2::{Cred, RemoteCallbacks};
use log::warn;

use crate::{cli::NewArgs, error::Result, template::clone_repository};

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
    let name = &args.name;
    let url = &args.template;
    let version = &args.version;
    let path = args.get_path()?;

    let callbacks: RemoteCallbacks = match classify_url(&args.template) {
        UrlKind::Http => http_callbacks(),
        UrlKind::Ssh => ssh_callbacks(),
        UrlKind::Invalid => return Err(format!("Invalid template url: '{}'", args.template).into())
    };

    clone_repository(url, &path, callbacks, version, &[
        ("EXTENSION-NAME", name.as_str()),
    ])
}


fn http_callbacks<'a>() -> RemoteCallbacks<'a> {
    let token = env::var("GITHUB_TOKEN");

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
        if let Ok(token) = &token {
            Cred::userpass_plaintext("", &token)
        } else {
            warn!("An HTTP URL was provided, but the GITHUB_TOKEN environment variable is empty. If the repository is private, it cannot be cloned");
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
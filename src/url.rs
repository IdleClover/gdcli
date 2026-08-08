use std::fmt;

#[derive(Clone)]
pub enum UrlKind {
    Http(String),
    Ssh(String),
    Invalid(String),
}

pub fn classify_url(url: &str) -> UrlKind {
    if url.starts_with("http://") || url.starts_with("https://") {
        UrlKind::Http(url.into())
    } else if url.starts_with("git@") || url.starts_with("ssh://") {
        UrlKind::Ssh(url.into())
    } else if let Some(url) = convert_gh(url) {
        UrlKind::Http(url)
    } else {
        UrlKind::Invalid(url.into())
    }
}

/// gh/user:name
pub fn convert_gh(url: &str) -> Option<String> {
    let short = url.strip_prefix("gh/")?;
    let mut parts = short.split(":");

    let user = parts.next()?;
    let name = parts.next()?;
    Some(format!("https://github.com/{}/{}.git", user, name))
}

impl UrlKind {
    pub fn url(&self) -> &str {
        match self {
            Self::Http(u) => u,
            Self::Ssh(u) => u,
            Self::Invalid(u) => u,
        }
    }
}

impl fmt::Display for UrlKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url())
    }
}

impl fmt::Debug for UrlKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url())
    }
}

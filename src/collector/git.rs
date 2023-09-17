use anyhow::{Context, Result};
use gix::remote::Direction;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoMetadata {
    // name: String,
    url: String,
}

pub fn get_repo_metadata(path: &Path) -> Result<RepoMetadata> {
    let repo = gix::open(path).context("Cannot open repo")?;

    let remote = repo
        .find_default_remote(Direction::Fetch)
        .context("Cannot find default remote")?
        .context("Cannot find default remote")?;

    let url = remote
        .url(Direction::Fetch)
        .context("Cannot find fetch url")?
        .to_bstring()
        .to_string();

    Ok(RepoMetadata { url })
}

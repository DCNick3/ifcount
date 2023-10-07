mod octocrab_ext;

use anyhow::{Context, Result};
use futures::{pin_mut, stream, StreamExt};
use indicatif::ProgressStyle;
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;
use tracing::{instrument, Span};
use tracing_indicatif::span_ext::IndicatifSpanExt;

use crate::collector::git::octocrab_ext::TreeItemType;
use crate::collector::File;
pub use octocrab_ext::LimitedCrab;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoMetadata {
    // name: String,
    pub url: String,
    pub commit: String,
}

fn progressbar_style() -> ProgressStyle {
    ProgressStyle::default_bar()
        .template("{span_child_prefix}{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta} @ {binary_bytes_per_sec})")
        .unwrap()
        .progress_chars("#>-")
}

#[instrument(skip(crab))]
pub async fn fetch_repo(
    crab: &LimitedCrab,
    repo_name: &str,
    commit: &str,
) -> Result<Vec<File<String>>> {
    let tree = crab
        .get_repo_tree(repo_name, &commit)
        .await
        .context("Cannot get repo tree")?;

    let wanted_files = tree
        .tree
        .into_iter()
        .filter(|i| i.type_ == TreeItemType::Blob)
        .filter(|i| i.path.ends_with(".rs"))
        // we don't want vendored dependencies
        .filter(|i| !i.path.starts_with("vendor/"))
        .collect::<Vec<_>>();

    let total_size = wanted_files.iter().map(|i| i.size.unwrap()).sum::<u64>();

    let cur_span = Span::current();

    cur_span.pb_set_style(&progressbar_style());
    cur_span.pb_set_length(total_size);

    let mut downloaded_files = Vec::with_capacity(wanted_files.len());

    let futures_stream = stream::iter(wanted_files.iter().map(|item| {
        let commit_hash = commit.to_owned();

        async move {
            let content = crab
                .get_file(repo_name, &commit_hash, item)
                .await
                .with_context(|| format!("Cannot get file {}", item.path))?;

            let file = File {
                path: RelativePathBuf::from(item.path.clone()),
                content,
            };

            Ok::<_, anyhow::Error>((item.size.unwrap(), file))
        }
    }))
    .buffer_unordered(16);
    pin_mut!(futures_stream);

    while let Some(result) = futures_stream.next().await {
        let (size, file) = result?;
        cur_span.pb_inc(size);
        downloaded_files.push(file);
    }

    Ok(downloaded_files)
}

#[instrument(skip(crab))]
pub async fn get_repo_metrics(
    crab: &LimitedCrab,
    repo_name: &str,
) -> Result<BTreeMap<String, serde_json::Value>> {
    let info = crab
        .get_repo_info(repo_name)
        .await
        .context("Cannot get repo info")?;

    let commit_count = crab.get_commit_count(repo_name).await?;

    let repo_metrics = json!(
        {
            "stars": info.stargazers_count.unwrap(),
            "watchers": info.watchers_count.unwrap(),
            "forks": info.forks_count.unwrap(),
            "open_issues": info.open_issues_count.unwrap(),
            "size": info.size.unwrap(),
            "commit_count": commit_count,
        }
    );

    Ok(BTreeMap::from([("repo".to_string(), repo_metrics)]))
}

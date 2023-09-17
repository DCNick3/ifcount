mod git;
mod metrics;

use crate::collector::git::RepoMetadata;
use anyhow::{Context, Result};
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::Path;
use tracing::{error, info, info_span};

pub struct File<T> {
    // path, relative to repo root
    path: RelativePathBuf,
    content: T,
}

pub type FileText = File<String>;
pub type FileAst = File<syn::File>;

impl FileText {
    #[tracing::instrument]
    pub fn read(repo_dir: &Path, path: &Path) -> Option<Self> {
        let Ok(path) = RelativePathBuf::from_path(path) else {
            error!("Filename is not UTF-8: `{}`", path.display());
            return None;
        };
        let content = match std::fs::read_to_string(path.to_path(repo_dir)) {
            Ok(v) => v,
            Err(e) => {
                error!("Error while reading: {:?}", e);
                return None;
            }
        };
        Some(Self { path, content })
    }

    #[tracing::instrument(skip(self), fields(path = %self.path))]
    pub fn parse(self) -> Option<FileAst> {
        let content = match syn::parse_file(&self.content) {
            Ok(v) => v,
            Err(e) => {
                error!("Error while parsing: {:?}", e);
                return None;
            }
        };
        Some(File {
            path: self.path,
            content,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoResult {
    meta: RepoMetadata,
    metrics: BTreeMap<String, serde_json::Value>,
}

// TODO: collect timings?
// pub struct Timings {
//     metrics: BTreeMap<String, Duration>,
// }

pub fn collect_repo(repo_path: &Path) -> Result<RepoResult> {
    let meta = git::get_repo_metadata(repo_path).context("Getting repo metadata")?;

    let collectors = metrics::get_metric_collectors();

    info!("Loading files from {}...", repo_path.display());
    let load_files_span = info_span!("load_files").entered();
    let files = ignore::WalkBuilder::new(repo_path)
        .sort_by_file_name(Ord::cmp)
        .require_git(true)
        .build()
        .filter_map(|v| v.map_err(|e| error!("Error during listing: {:?}", e)).ok())
        .filter(|v| v.path().extension() == Some(OsStr::new("rs")))
        .filter(|v| v.file_type().is_some_and(|t| t.is_file()))
        .filter_map(|v| {
            let path =
                pathdiff::diff_paths(v.path(), repo_path).expect("BUG: found path not in repo");
            File::read(repo_path, &path)
        })
        .filter_map(File::parse)
        .collect::<Vec<_>>();
    load_files_span.exit();

    info!("Collecting metrics from {} files...", files.len());
    let collect_metrics_span = info_span!("collect_metrics").entered();
    let metrics = collectors
        .iter()
        .map(|collector| {
            (
                collector.name().to_string(),
                collector.collect_metric(&files),
            )
        })
        .collect::<BTreeMap<_, _>>();
    collect_metrics_span.exit();

    Ok(RepoResult { meta, metrics })
}

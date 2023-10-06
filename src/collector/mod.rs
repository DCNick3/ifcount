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

pub use git::LimitedCrab;

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
    pub meta: RepoMetadata,
    pub metrics: BTreeMap<String, serde_json::Value>,
}

// TODO: collect timings?
// pub struct Timings {
//     metrics: BTreeMap<String, Duration>,
// }

fn count_submetrics(value: &serde_json::Value) -> usize {
    use serde_json::Value;

    match value {
        Value::Bool(_) | Value::String(_) | Value::Array(_) => {
            panic!("Unknown type encountered in metrics: {}", value)
        }
        // null can be encountered in histogram's `mode` field
        Value::Null | Value::Number(_) => 1,
        Value::Object(obj) => obj.values().map(count_submetrics).sum::<usize>(),
    }
}

fn count_metrics(metrics: &BTreeMap<String, serde_json::Value>) -> usize {
    metrics.values().map(count_submetrics).sum::<usize>()
}

fn get_submetric_list(pre_path: &mut String, result: &mut Vec<String>, value: &serde_json::Value) {
    use serde_json::Value;

    match value {
        Value::Bool(_) | Value::String(_) | Value::Array(_) => {
            panic!("Unknown type encountered in metrics: {}", value)
        }
        Value::Null | Value::Number(_) => result.push(pre_path.clone()),
        Value::Object(obj) => {
            for (name, value) in obj.iter() {
                pre_path.push('.');
                pre_path.push_str(name);
                get_submetric_list(pre_path, result, value);

                for _ in 0..name.len() + 1 {
                    pre_path.pop();
                }
            }
        }
    }
}

pub fn get_metric_list(metrics: &BTreeMap<String, serde_json::Value>) -> Vec<String> {
    let mut result = Vec::new();
    let mut pre_path = String::new();

    for (name, value) in metrics {
        pre_path.push_str(name);
        get_submetric_list(&mut pre_path, &mut result, value);
        pre_path.clear();
    }

    result
}

fn collect_file_metrics(files: &[FileAst]) -> Result<BTreeMap<String, serde_json::Value>> {
    let collectors = metrics::get_metric_collectors();

    info!("Collecting metrics from {} files...", files.len());
    let collect_metrics_span = info_span!("collect_metrics").entered();
    let metrics = collectors
        .iter()
        .map(|collector| {
            (
                collector.name().to_string(),
                collector.collect_metric(files),
            )
        })
        .collect::<BTreeMap<_, _>>();
    collect_metrics_span.exit();

    info!("Collected {} repo metrics!", count_metrics(&metrics));

    Ok(metrics)
}

pub fn collect_local_repo(repo_path: &Path) -> Result<RepoResult> {
    let meta = git::get_repo_metadata(repo_path).context("Getting repo metadata")?;

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

    let metrics = collect_file_metrics(&files)?;

    Ok(RepoResult { meta, metrics })
}

pub async fn collect_github_repo(crab: &LimitedCrab, repo_name: &str) -> Result<RepoResult> {
    info!("Downloading https://github.com/{}...", repo_name);

    let files = git::fetch_repo(crab, repo_name)
        .await
        .context("Fetching repo")?;

    let load_files_span = info_span!("parse_files").entered();
    let files = files
        .into_iter()
        .filter_map(File::parse)
        .collect::<Vec<_>>();
    load_files_span.exit();

    let mut metrics = collect_file_metrics(&files)?;
    metrics.extend(
        git::get_repo_metrics(crab, repo_name)
            .await
            .context("Getting repo metrics")?,
    );
    info!("Collected {} total metrics!", count_metrics(&metrics));

    let meta = RepoMetadata {
        url: format!("git@github.com:{}.git", repo_name),
    };

    Ok(RepoResult { meta, metrics })
}

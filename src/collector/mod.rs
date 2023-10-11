mod git;
mod metrics;
mod rust_code_analysis;

use crate::collector::git::RepoMetadata;
use ::rust_code_analysis::{ParserTrait, RustParser};
use anyhow::{Context, Result};
use indicatif::ProgressStyle;
use rayon::prelude::*;
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::ops::Deref;
use std::path::Path;
use tracing::{error, info, info_span, instrument, Span};
use tracing_indicatif::span_ext::IndicatifSpanExt;

pub use git::LimitedCrab;

use self::rust_code_analysis::RCAMetrics;

#[derive(Clone)]
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

    #[tracing::instrument(skip(self, span), parent = span, fields(path = %self.path))]
    pub fn parse(self, span: Span) -> Option<FileAst> {
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

fn flatten_submetrics(
    pre_path: &mut String,
    result: &mut BTreeMap<String, serde_json::Value>,
    value: &serde_json::Value,
) {
    use serde_json::Value;

    match value {
        Value::Bool(_) | Value::String(_) | Value::Array(_) => {
            panic!("Unknown type encountered in metrics: {}", value)
        }
        Value::Null | Value::Number(_) => {
            result.insert(pre_path.clone(), value.clone());
        }
        Value::Object(obj) => {
            for (name, value) in obj.iter() {
                pre_path.push('.');
                pre_path.push_str(name);
                flatten_submetrics(pre_path, result, value);

                for _ in 0..name.len() + 1 {
                    pre_path.pop();
                }
            }
        }
    }
}

pub fn flatten_metrics(
    metrics: &BTreeMap<String, serde_json::Value>,
) -> BTreeMap<String, serde_json::Value> {
    let mut result = BTreeMap::new();
    let mut pre_path = String::new();

    for (name, value) in metrics {
        pre_path.push_str(name);
        flatten_submetrics(&mut pre_path, &mut result, value);
        pre_path.clear();
    }

    result
}

fn collect_file_metrics(files: &[FileAst]) -> Result<BTreeMap<String, serde_json::Value>> {
    let collectors = metrics::get_metric_collectors();

    info!("Collecting metrics from {} files...", files.len());
    let collect_metrics_span = info_span!("collect_metrics").entered();
    let metrics = collectors
        // I would __like__ to use `par_iter`, but we hit deadlocks for some reason..
        .iter()
        .map(|collector| {
            let _span = info_span!(parent: collect_metrics_span.id(), "collect_metric", metric = collector.name()).entered();

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
    // we could have implemented it with gix, but it's a large dep for minor gains
    let meta = RepoMetadata {
        url: "<LOCAL>".to_string(),
        commit: "<LOCAL>".to_string(),
    };

    info!("Loading files from {}...", repo_path.display());
    let load_files_span = info_span!("load_files").entered();
    let raw_files = ignore::WalkBuilder::new(repo_path)
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
        .collect::<Vec<_>>();

    let files = raw_files
        .clone()
        .into_par_iter()
        .filter_map(|f| File::parse(f, Span::current()))
        .collect::<Vec<_>>();
    load_files_span.exit();

    let rust_analysis_metrics = collect_rust_code_analysis(&raw_files)?;
    let mut metrics = collect_file_metrics(&files)?;
    metrics.extend(rust_analysis_metrics);
    let metrics = flatten_metrics(&metrics);

    Ok(RepoResult { meta, metrics })
}

pub fn collect_rust_code_analysis(
    files: &[File<String>],
) -> Result<BTreeMap<String, serde_json::Value>> {
    let byte_sources: Vec<_> = files
        .into_iter()
        .map(|x| (&x.path, x.content.as_bytes().to_vec()))
        .collect();

    let file_metrics: Vec<_> = byte_sources
        .into_par_iter()
        .flat_map(|(path, contents)| {
            let parser = RustParser::new(contents.clone(), &path.to_path(""), None);
            ::rust_code_analysis::metrics(&parser, &path.to_path(""))
                .context("Extracting rust_code_analysis metrics")
                .map_err(|e| error!("{e}"))
                .map(|space| (path, space))
        })
        .map(|(path, mut space)| {
            space.spaces.clear();
            (path.to_string(), space)
        })
        .map(|(_, space)| space.metrics)
        .collect();

    let mut statisics = RCAMetrics::default();
    for metrics_batch in file_metrics {
        statisics.observe(&metrics_batch);
    }

    let metrics = serde_json::to_value(statisics)?;
    Ok(BTreeMap::from([("rca".to_string(), metrics)]))
}

#[instrument(skip(crab))]
pub async fn collect_github_repo(crab: &LimitedCrab, repo_name: &str) -> Result<RepoResult> {
    info!("Downloading https://github.com/{}...", repo_name);

    let commit = crab.get_latest_commit(repo_name).await?;

    let text_files = git::fetch_repo(crab, repo_name, &commit)
        .await
        .context("Fetching repo")?;

    let files = tokio::task::block_in_place(|| {
        let span = info_span!("parse_files").entered();

        text_files
            .clone()
            .into_par_iter()
            .filter_map(|f| File::parse(f, span.deref().clone()))
            .collect::<Vec<_>>()
    });

    let mut metrics = tokio::task::block_in_place(move || collect_file_metrics(&files))?;
    let rca_metrics = tokio::task::block_in_place(|| collect_rust_code_analysis(&text_files))?;
    metrics.extend(rca_metrics);
    metrics.extend(
        git::get_repo_metrics(crab, repo_name)
            .await
            .context("Getting repo metrics")?,
    );

    let metrics = flatten_metrics(&metrics);

    info!("Collected {} total metrics!", metrics.len());

    let meta = RepoMetadata {
        url: format!("git@github.com:{}.git", repo_name),
        commit,
    };

    Ok(RepoResult { meta, metrics })
}

fn progressbar_style() -> ProgressStyle {
    ProgressStyle::default_bar()
        .template("{span_child_prefix}{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len}")
        .unwrap()
        .progress_chars("#>-")
}

#[instrument(skip(crab, repo_list))]
pub async fn bulk_collect_github_repos(
    crab: &LimitedCrab,
    repo_list: &[&str],
) -> Result<Vec<RepoResult>> {
    let span = Span::current();
    span.pb_set_style(&progressbar_style());
    span.pb_set_length(repo_list.len() as u64);

    let mut results = Vec::with_capacity(repo_list.len());

    for repo_name in repo_list {
        let result = collect_github_repo(crab, repo_name)
            .await
            .with_context(|| format!("Collecting metrics for {}", repo_name))?;

        span.pb_inc(1);

        results.push(result);
    }

    Ok(results.into_iter().collect::<Vec<_>>())
}

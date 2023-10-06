mod collector;

use crate::collector::LimitedCrab;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use std::io::Write;
use std::path::PathBuf;
use tracing::warn;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

const BANNER: &str = r#"
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣠⣴⣾⡇⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⣠⣶⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣷⣿⠀⠀⠀
⠀⢀⣠⣴⣾⡇⠀⢸⣿⣿⣿⣯⣤⡄⠀⣀⣴⣾⣿⣿⣿⣿⣷⣦⡀⠀⠀⠀⢀⣤⣴⣶⣶⣤⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣿⣿⣿⣿⣿⣿⠋⠀⠀⠀
⠀⣿⠿⢛⣩⡄⢠⣾⣿⣿⣿⣿⡿⠇⢰⣿⣿⣿⣿⡏⢹⣿⣿⣿⣿⡀⠀⣰⣿⣿⣿⣿⠿⣿⣿⣿⣷⣄⠀⠀⢸⣿⣷⣶⣶⡆⠀⣤⣤⣤⣤⣤⣄⠀⠀⣤⣤⣤⣤⣤⡄⣤⣾⣿⣿⣿⣷⡄⠀⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀
⠀⣵⣾⣿⣿⡇⠸⣻⣿⣿⣿⣿⡇⠀⣿⣿⣿⣿⣿⠀⢸⣿⣿⣿⣿⡇⠀⣿⣿⣿⣿⣿⠀⢸⣿⣿⣿⣿⡆⠀⢸⣿⣿⣿⣿⡇⠀⢻⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⡿⠿⣿⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀
⠀⣿⣿⣿⣿⡇⠀⣿⣿⣿⣿⣿⠁⠀⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⠀⢸⣿⣿⣿⣿⡇⠀⢺⣿⣿⣿⣿⡇⠀⢸⣿⣿⣿⣿⡿⠀⠀⣿⣿⣿⣿⣿⡇⠀⣿⣿⣿⣿⣿⣿⠀⠀⢸⣿⣿⣿⣿⣿⠀⠀⠀⠀
⢰⣿⣿⣿⣿⡇⠀⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⠀⠀⣀⣀⣀⠀⠀⠈⣿⣿⣿⣿⣿⠀⢸⣿⣿⣿⣿⡇⠀⢸⣿⣿⣿⣿⡇⠀⢸⣿⣿⣿⣿⡇⠀⠀⣿⣿⣿⣿⣿⡇⠀⢻⣿⣿⣿⣿⣿⠀⠀⢸⣿⣿⣿⣿⣿⠀⠀⠀⠀
⢸⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⠃⠀⣿⣿⣿⣿⣿⠀⢸⣿⣿⣿⣿⡇⠀⢸⣿⣿⣿⣿⡇⠀⢸⣿⣿⣿⣿⣧⠀⠀⣿⣿⣿⣿⣿⡇⠀⢸⣿⣿⣿⣿⣿⠀⠀⠸⣿⣿⣿⣿⣿⠀⠀⠀⠀
⢸⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⠀⠀⢹⣿⣿⣿⣿⣄⣸⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣯⠀⢸⣿⣿⣿⣿⡇⠀⢸⣿⣿⣿⣿⡇⠀⢸⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⡇⠀⢸⣿⣿⣿⣿⣿⠀⠀⢠⣿⣿⣿⣿⣿⣷⣿⠀⠀
⢸⣿⣿⣿⡿⠀⠀⣿⣿⣿⣿⡏⠀⠀⠈⠻⠿⣿⣿⣿⣿⣿⡿⠟⠁⠀⠀⠘⢿⣿⣿⣿⣦⣾⣿⣿⣿⣿⠃⠀⢸⣿⣿⣿⣿⡇⠀⢸⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⡇⠀⢸⣿⣿⣿⣿⣿⠀⠀⠈⣿⣿⣿⣿⡿⠟⠋⠀⠀
⠘⠙⠛⠛⠇⠀⠀⠿⠛⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠛⠻⠿⠿⠟⠛⠁⠀⠀⠀⠛⠛⠛⠛⠓⠤⠜⠛⠛⠛⠛⠋⠀⠀⠸⠛⠛⠛⠛⠇⠀⠈⠟⠛⠛⠛⠛⠀⠀⠀⠈⠉⠉⠀⠀⠀⠀⠀⠀

"#;

#[derive(Debug, Subcommand)]
enum CliCommand {
    /// Collect metrics from a checked out repository
    CollectLocalRepo {
        /// Path to the repository to collect metrics for
        repo_path: PathBuf,
    },
    /// Collect metrics from a github repository
    CollectGithubRepo {
        repo_name: String,
    },
    BulkCollectGithubRepos {
        list_path: PathBuf,
    },
    /// Get a list of supported metrics
    ///
    /// Work internally by running against `DCNick3/ifcount`
    ListMetrics {
        #[clap(long)]
        latex: bool,
    },
}

async fn make_crab(dirs: &ProjectDirs) -> Result<LimitedCrab> {
    let token = if let Ok(github_token) = std::env::var("GITHUB_TOKEN") {
        Some(github_token)
    } else {
        // most of the time we'll be talking to `raw.githubusercontent.com`, so it's not that bad if we don't have a token
        warn!("GITHUB_TOKEN not set, not authenticating when talking to GitHub API");
        None
    };
    let cache_path = dirs.cache_dir().join("gh-cache");

    LimitedCrab::new(token, cache_path)
        .await
        .context("Creating octocrab")
}

impl CliCommand {
    pub async fn run(self, dirs: &ProjectDirs) -> Result<()> {
        match self {
            CliCommand::CollectLocalRepo { repo_path } => {
                let result = collector::collect_local_repo(&repo_path)?;

                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).context("Serializing results")?
                );

                Ok(())
            }
            CliCommand::CollectGithubRepo { repo_name } => {
                let crab = make_crab(dirs).await?;

                let result = collector::collect_github_repo(&crab, &repo_name)
                    .await
                    .context("Collecting metrics")?;

                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).context("Serializing results")?
                );

                Ok(())
            }
            CliCommand::ListMetrics { latex } => {
                let crab = make_crab(dirs).await?;

                let result = collector::collect_github_repo(&crab, "DCNick3/ifcount")
                    .await
                    .context("Collecting metrics")?;

                let metric_list = collector::get_metric_list(&result.metrics);

                for metric in metric_list {
                    if latex {
                        println!(
                            "\\metric{{{}}}{{INSERT HERE}}",
                            crowbook_text_processing::escape::tex(&metric)
                        );
                    } else {
                        println!("{}", metric);
                    }
                }

                Ok(())
            }
            CliCommand::BulkCollectGithubRepos { list_path } => {
                let crab = make_crab(dirs).await?;

                let repo_list = std::fs::read_to_string(&list_path).context("Reading repo list")?;
                let repo_list = repo_list
                    .lines()
                    .map(|line| line.trim())
                    .filter(|line| !line.is_empty())
                    .collect::<Vec<_>>();

                let results = collector::bulk_collect_github_repos(&crab, &repo_list)
                    .await
                    .context("Collecting metrics")?;

                println!(
                    "{}",
                    serde_json::to_string_pretty(&results).context("Serializing results")?
                );

                Ok(())
            }
        }
    }
}

/// Collect metrics for Rust code
#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: CliCommand,
}

const DEFAULT_ENV_FILTER: &str = "info";

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(windows)]
    let _enabled = ansi_term::enable_ansi_support();

    let indicatif_layer = IndicatifLayer::new();
    let mut stderr = indicatif_layer.get_stderr_writer();

    tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(DEFAULT_ENV_FILTER))
        .with_subscriber(
            tracing_subscriber::registry()
                .with(tracing_subscriber::fmt::layer().with_writer(stderr.clone()))
                .with(indicatif_layer),
        )
        .init();
    write!(&mut stderr, "{}", BANNER).context("Writing banner")?;

    let cli = Cli::parse();

    let dirs = ProjectDirs::from("me.dcnick3", "NINIKA Company", "ifcount")
        .context("Getting project directories")?;

    cli.command.run(&dirs).await
}

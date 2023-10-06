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
    CollectGithubRepo { repo_name: String },
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
                let token = if let Ok(github_token) = std::env::var("GITHUB_TOKEN") {
                    Some(github_token)
                } else {
                    // most of the time we'll be talking to `raw.githubusercontent.com`, so it's not that bad if we don't have a token
                    warn!("GITHUB_TOKEN not set, not authenticating when talking to GitHub API");
                    None
                };
                let cache_path = dirs.cache_dir().join("gh-cache");
                let crab = LimitedCrab::new(token, cache_path)
                    .await
                    .context("Creating octocrab")?;

                let result = collector::collect_github_repo(&crab, &repo_name)
                    .await
                    .context("Collecting metrics")?;

                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).context("Serializing results")?
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

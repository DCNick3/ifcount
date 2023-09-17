mod collector;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::io::Write;
use std::path::PathBuf;
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
    /// Collect metrics for a checked out repository
    CollectRepo {
        /// Path to the repository to collect metrics for
        repo_path: PathBuf,
    },
}

impl CliCommand {
    pub fn run(self) -> Result<()> {
        match self {
            CliCommand::CollectRepo { repo_path } => {
                let result = collector::collect_repo(&repo_path)?;

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

fn main() -> Result<()> {
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

    cli.command.run()
}

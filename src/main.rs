use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;

mod client;
mod comparator;
mod config;
mod report;
mod runner;

use client::GrpcClient;
use config::Config;
use report::ReportGenerator;
use runner::TestRunner;

#[derive(Parser)]
#[command(name = "cts-gate")]
#[command(about = "Zcash backend conformance testing tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run test suite
    Run {
        /// Backend profile name (e.g., lightwalletd)
        #[arg(long)]
        profile: String,

        /// Test suite to run (e.g., mvp)
        #[arg(long)]
        suite: String,

        /// Output directory for reports
        #[arg(long, default_value = "reports")]
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            profile,
            suite,
            output,
        } => {
            run_tests(&profile, &suite, &output).await?;
        }
    }

    Ok(())
}

async fn run_tests(profile: &str, suite: &str, output_dir: &std::path::Path) -> Result<()> {
    info!("Loading profile: {}", profile);
    let config = Config::load_profile(profile)?;

    info!("Loading test suite: {}", suite);
    let test_cases = config.load_test_cases(suite)?;

    info!("Connecting to backend...");
    let client = GrpcClient::connect(&config.endpoint).await?;

    info!("Running {} test cases...", test_cases.len());
    let runner = TestRunner::new(client);
    let results = runner.run_tests(&test_cases).await?;

    info!("Generating reports...");
    let generator = ReportGenerator::new();
    generator.generate_reports(&results, output_dir)?;

    let all_passed = results.iter().all(|r| r.passed);
    if all_passed {
        info!("All tests PASSED ✅");
        Ok(())
    } else {
        let failed = results.iter().filter(|r| !r.passed).count();
        eprintln!("Tests FAILED: {}/{} cases failed ❌", failed, results.len());
        std::process::exit(1);
    }
}

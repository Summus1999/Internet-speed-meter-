use clap::Parser;
use std::time::Instant;
use anyhow::Result;

mod speed_test;
mod diagnostics;
mod utils;

use speed_test::SpeedTester;
use diagnostics::NetworkDiagnostics;

#[derive(Parser, Debug)]
#[command(name = "Internet Speed Meter")]
#[command(about = "A tool for measuring internet speed and diagnosing network issues", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Run speed test
    Speed {
        /// Test server URL (optional)
        #[arg(short, long)]
        server: Option<String>,
    },
    /// Run network diagnostics
    Diagnose {
        /// Target host to diagnose
        #[arg(long, default_value = "8.8.8.8")]
        host: String,
    },
    /// Run all tests
    All {
        #[arg(short, long)]
        server: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Speed { server } => run_speed_test(server).await?,
        Commands::Diagnose { host } => run_diagnostics(&host).await?,
        Commands::All { server } => run_all_tests(server).await?,
    }

    Ok(())
}

async fn run_speed_test(server: Option<String>) -> Result<()> {
    println!("\n🚀 Starting Speed Test...");
    let start = Instant::now();

    let tester = SpeedTester::new(server);
    let results = tester.run_test().await?;

    let elapsed = start.elapsed();
    results.print_summary();
    println!("\n⏱️  Test completed in {:.2}s", elapsed.as_secs_f64());

    Ok(())
}

async fn run_diagnostics(host: &str) -> Result<()> {
    println!("\n🔍 Running Network Diagnostics...");
    let start = Instant::now();

    let diagnostics = NetworkDiagnostics::new();
    let results = diagnostics.run_full_diagnostic(host).await?;

    let elapsed = start.elapsed();
    results.print_report();
    println!("\n⏱️  Diagnostics completed in {:.2}s", elapsed.as_secs_f64());

    Ok(())
}

async fn run_all_tests(server: Option<String>) -> Result<()> {
    println!("\n📊 Running Complete Network Analysis...");
    let start = Instant::now();

    // Run speed test
    println!("\n--- Speed Test ---");
    let tester = SpeedTester::new(server);
    let speed_results = tester.run_test().await?;
    speed_results.print_summary();

    // Run diagnostics
    println!("\n--- Network Diagnostics ---");
    let diagnostics = NetworkDiagnostics::new();
    let diag_results = diagnostics.run_full_diagnostic("8.8.8.8").await?;
    diag_results.print_report();

    let elapsed = start.elapsed();
    println!("\n✅ Complete analysis finished in {:.2}s", elapsed.as_secs_f64());

    Ok(())
}

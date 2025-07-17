// src/main.rs
/*
 * Main executable for ChaincodeDeployer
 */

use clap::Parser;
use chaincodedeployer::{Result, run};

#[derive(Parser)]
#[command(version, about = "ChaincodeDeployer - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}

// src/main.rs
/*
 * Main executable for PrimeOptimo
 */

use clap::Parser;
use primeoptimo::{Result, run};

#[derive(Parser)]
#[command(version, about = "PrimeOptimo - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}

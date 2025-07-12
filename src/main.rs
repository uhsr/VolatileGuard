// src/main.rs
/*
 * Main executable for VolatileGuard
 */

use clap::Parser;
use volatileguard::{Result, run};

#[derive(Parser)]
#[command(version, about = "VolatileGuard - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}

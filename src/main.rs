// src/main.rs
/*
 * Main executable for NonFungibleTokenizer
 */

use clap::Parser;
use nonfungibletokenizer::{Result, run};

#[derive(Parser)]
#[command(version, about = "NonFungibleTokenizer - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}

use std::process::ExitCode;

use clap::Parser;
use runner::{Args, Runner};

fn main() -> ExitCode {
    let args = Args::parse();
    Runner::try_from(args).unwrap_or_else(|e| e.exit()).run()
}

use std::process::ExitCode;

use clap::Parser;
use runner::Cli;

fn main() -> ExitCode {
    Cli::parse().run()
}

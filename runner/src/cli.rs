use crate::args::{daily, download, run};
use clap::{Parser, Subcommand};
use std::process::{ExitCode, ExitStatus};

pub fn exit_code_from_status(stauts: ExitStatus) -> ExitCode {
    if stauts.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// Run an Advent of Code solution.
#[derive(Parser)]
pub struct Cli {
    /// The subcommand to run. Defaults to `run`.
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run an Advent of Code solution. This is the default subcommand.
    Run(run::Args),

    /// Scaffold a new Advent of Code solution for a single day.
    Daily(daily::Args),

    /// Download the puzzle input for a given year and day.
    Download(download::Args),
}

impl Default for Command {
    fn default() -> Self {
        Self::Run(Default::default())
    }
}

impl Cli {
    pub fn run(self) -> ExitCode {
        let command = self.command.unwrap_or_default();
        match command {
            Command::Run(args) => args.run(),
            Command::Daily(args) => args.run(),
            Command::Download(args) => args.run(),
        }
        .unwrap_or_else(|e| e.exit())
    }
}

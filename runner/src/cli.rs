use std::process::{ExitCode, ExitStatus};

use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, Subcommand};

use crate::is_aoc_event_for;
use crate::types::{Day, Year};

fn exit_code_from_status(stauts: ExitStatus) -> ExitCode {
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
    Run(RunArgs),
}

impl Default for Command {
    fn default() -> Self {
        Self::Run(RunArgs {
            year: None,
            day: None,
            debug: false,
        })
    }
}

#[derive(clap::Args, Default)]
pub struct RunArgs {
    /// The year of the challenge to run.
    ///
    /// This is required if the current date is not during the Advent of Code event.
    year: Option<Year>,

    /// The day of the challenge to run.
    ///
    /// This is required if the current date is not during the Advent of Code event.
    day: Option<Day>,

    /// Whether to run the solution in debug mode.
    #[clap(short, long, default_value_t = false)]
    debug: bool,
}

impl RunArgs {
    pub fn year(&self) -> Result<Year, &str> {
        self.year.map_or_else(|| {
            let now = chrono::Local::now();
            if is_aoc_event_for(now) {
                Ok(now.into())
            } else {
                return Err("the year is required if the current date is not during the Advent of Code event");
            }
        }, Ok)
    }

    pub fn day(&self) -> Result<Day, &str> {
        self.day.map_or_else(|| {
            let now = chrono::Local::now();
            if is_aoc_event_for(now) {
                Ok(now.into())
            } else {
                return Err("the day is required if the current date is not during the Advent of Code event");
            }
        }, Ok)
    }

    pub fn run(&self) -> Result<ExitCode, clap::Error> {
        let year = self
            .year()
            .map_err(|e| Cli::command().error(ErrorKind::MissingRequiredArgument, e))?;

        let day = self
            .day()
            .map_err(|e| Cli::command().error(ErrorKind::MissingRequiredArgument, e))?;

        eprintln!("Running year {}, day {}...", year, day);
        let mut command = std::process::Command::new("cargo");
        command.arg("run");

        if !self.debug {
            command.arg("--release");
        }

        command
            .arg("--quiet")
            .arg("--bin")
            .arg(format!("year{}-day{:02}", year, day))
            .status()
            .map(exit_code_from_status)
            .map_err(|e| Cli::command().error(ErrorKind::Io, e))
    }
}

impl Cli {
    pub fn run(self) -> ExitCode {
        let command = &self.command.unwrap_or_else(|| Command::default());

        match command {
            Command::Run(args) => args.run(),
        }
        .unwrap_or_else(|e| e.exit())
    }
}

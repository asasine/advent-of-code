use std::process::{Command, ExitCode, ExitStatus};

use clap::{error::ErrorKind, CommandFactory};

use crate::types::{Day, Year};
use crate::Args;

fn exit_code_from_status(stauts: ExitStatus) -> ExitCode {
    if stauts.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

pub struct Runner {
    year: Year,
    day: Day,
    debug: bool,
}

impl Runner {
    /// Runs the solution for the given year and day.
    pub fn run(&self) -> ExitCode {
        eprintln!("Running year {}, day {}...", self.year, self.day);

        let mut command = Command::new("cargo");
        command.arg("run");

        if !self.debug {
            command.arg("--release");
        }

        command
            .arg("--quiet")
            .arg("--bin")
            .arg(format!("year{}-day{:02}", self.year.0, self.day.0))
            .status()
            .map(exit_code_from_status)
            .expect("failed to run the solution")
    }
}

impl TryFrom<Args> for Runner {
    type Error = clap::Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let year = args
            .year()
            .map_err(|e| Args::command().error(ErrorKind::MissingRequiredArgument, e))?;

        let day = args
            .day()
            .map_err(|e| Args::command().error(ErrorKind::MissingRequiredArgument, e))?;

        Ok(Self {
            year,
            day,
            debug: args.debug,
        })
    }
}

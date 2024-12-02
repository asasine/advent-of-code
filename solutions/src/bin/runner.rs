use std::fmt;
use std::process::{Command, ExitCode, ExitStatus};
use std::str::FromStr;

use chrono::Datelike;
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};

/// Returns `true` if `dt` is during the Advent of Code event.
fn is_aoc_event_for(dt: impl Datelike) -> bool {
    dt.month() == 12 && dt.day() <= 25
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Year(u16);

impl Default for Year {
    fn default() -> Self {
        let now = chrono::Local::now();
        let year = now.year();
        if now.month() == 12 {
            Self(year as u16)
        } else {
            Self(year as u16 - 1)
        }
    }
}

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Year {
    type Err = <u16 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u16::from_str(s).map(Self)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Day(u8);

impl Default for Day {
    fn default() -> Self {
        let now = chrono::Local::now();
        let day = now.day();
        if now.month() == 12 {
            Self(day as u8)
        } else {
            Self(25)
        }
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Day {
    type Err = <u8 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u8::from_str(s).map(Self)
    }
}

/// Run an Advent of Code solution.
#[derive(Parser)]
struct Args {
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

impl Args {
    fn year(&self) -> Result<Year, &str> {
        self.year.map_or_else(|| {
            let now = chrono::Local::now();
            if is_aoc_event_for(now) {
                Ok(Year(now.year() as u16))
            } else {
                return Err("the year is required if the current date is not during the Advent of Code event");
            }
        }, Ok)
    }

    fn day(&self) -> Result<Day, &str> {
        self.day.map_or_else(|| {
            let now = chrono::Local::now();
            if is_aoc_event_for(now) {
                Ok(Day(now.day() as u8))
            } else {
                return Err("the day is required if the current date is not during the Advent of Code event");
            }
        }, Ok)
    }
}

fn exit_code_from_status(stauts: ExitStatus) -> ExitCode {
    if stauts.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

struct Runner {
    year: Year,
    day: Day,
    debug: bool,
}

impl Runner {
    /// Runs the solution for the given year and day.
    fn run(&self) -> ExitCode {
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

fn main() -> ExitCode {
    let args = Args::parse();
    Runner::try_from(args).unwrap_or_else(|e| e.exit()).run()
}

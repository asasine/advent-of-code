use std::fs::File;
use std::path::PathBuf;
use std::process::{ExitCode, ExitStatus, Stdio};

use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{Args, CommandFactory, Parser, Subcommand};

use crate::is_aoc_event_for;
use crate::types::{Day, Year, YearDay};

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

    /// Scaffold a new Advent of Code solution for a single day.
    Daily(DailyArgs),
}

impl Default for Command {
    fn default() -> Self {
        Self::Run(RunArgs::default())
    }
}

#[derive(Args, Default)]
struct YearDayArgs {
    /// The year of the challenge.
    ///
    /// This is required if the current date is not during the Advent of Code event.
    year: Option<Year>,

    /// The day of the challenge.
    ///
    /// This is required if the current date is not during the Advent of Code event.
    day: Option<Day>,
}

impl YearDayArgs {
    pub fn year(&self) -> Result<Year, &str> {
        self.year.map_or_else(|| {
            let now = chrono::Local::now();
            if is_aoc_event_for(now) {
                Ok(now.into())
            } else {
                Err("the year is required if the current date is not during the Advent of Code event")
            }
        }, Ok)
    }

    pub fn day(&self) -> Result<Day, &str> {
        self.day.map_or_else(|| {
            let now = chrono::Local::now();
            if is_aoc_event_for(now) {
                Ok(now.into())
            } else {
                Err("the day is required if the current date is not during the Advent of Code event")
            }
        }, Ok)
    }

    fn validate(&self) -> Result<YearDay, clap::Error> {
        let year = self
            .year()
            .map_err(|e| Cli::command().error(ErrorKind::MissingRequiredArgument, e))?;

        let day = self
            .day()
            .map_err(|e| Cli::command().error(ErrorKind::MissingRequiredArgument, e))?;

        Ok(YearDay { year, day })
    }
}

#[derive(Args, Default)]
pub struct RunArgs {
    #[clap(flatten)]
    yd: YearDayArgs,

    /// Whether to run the solution in debug mode.
    #[clap(short, long, default_value_t = false)]
    debug: bool,
}

impl RunArgs {
    /// Runs the solution for the given year and day.
    ///
    /// This reads the puzzle input from the file system and feeds it to the stdin of the solution binary.
    pub fn run(self) -> Result<ExitCode, clap::Error> {
        let yd = self.yd.validate()?;
        let &YearDay { year, day } = &yd;
        eprintln!("Running year {}, day {}...", year, day);
        let input = self.get_puzzle_input(&yd)?;
        let mut command = self.get_command(&yd, input);
        command
            .status()
            .map(exit_code_from_status)
            .map_err(|e| Cli::command().error(ErrorKind::Io, e))
    }

    /// Gets a file handle to the puzzle input for the given year and day.
    fn get_puzzle_input(&self, yd: &YearDay) -> Result<File, clap::Error> {
        let p = PathBuf::from("solutions/data/real").join(format!("{}/{:02}.txt", yd.year, yd.day));
        File::open(p).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => Cli::command().error(
                ErrorKind::Io,
                format!("no puzzle input found for year {}, day {}", yd.year, yd.day),
            ),
            _ => e.into(),
        })
    }

    /// Gets a command to run the solution for the given year and day.
    ///
    /// This does not run the command, but returns a [`std::process::Command`] that can be used to run the solution.
    ///
    /// The command will be run in the current working directory. Its stdin will be connected to the puzzle input file
    /// as provided by the `stdin` argument.
    fn get_command<T: Into<Stdio>>(&self, yd: &YearDay, stdin: T) -> std::process::Command {
        let mut command = std::process::Command::new("cargo");
        command.arg("run");

        if !self.debug {
            command.arg("--release");
        }

        command
            .arg("--quiet")
            .arg("--bin")
            .arg(format!("year{}-day{:02}", yd.year, yd.day))
            .stdin(stdin);

        command
    }
}

#[derive(clap::Args, Default)]
pub struct DailyArgs {
    #[clap(flatten)]
    yd: YearDayArgs,

    /// The file to write the output to.
    ///
    /// If not provided, the destination will be automatically derived from the year and day.
    destination: Option<PathBuf>,

    /// Whether to overwrite the output file if it already exists.
    #[clap(short, long, default_value_t = false)]
    force: bool,
}

impl DailyArgs {
    fn contents(yd: &YearDay) -> String {
        let doc = format!("Day {}", yd.day);
        let real_input_include_str = format!("../../data/real/{}/{:02}.txt", yd.year, yd.day);
        let example_input_include_str =
            format!("../../data/examples/{}/{:02}/1.txt", yd.year, yd.day);

        format!(
            r#"//! {doc}

fn part1(input: &str) -> usize {{
    0
}}

fn part2(input: &str) -> usize {{
    0
}}

fn main() {{
    let input = include_str!("{real_input_include_str}");
    println!("{{}}", part1(input));
    println!("{{}}", part2(input));
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn part1_example() {{
        let input = include_str!("{example_input_include_str}");
        assert_eq!(0, part1(input));
    }}

    #[test]
    fn part2_example() {{
        let input = include_str!("{example_input_include_str}");
        assert_eq!(0, part2(input));
    }}
}}
"#
        )
    }

    pub fn run(self) -> Result<ExitCode, clap::Error> {
        let yd = self.yd.validate()?;
        eprintln!("Scaffolding for year {}, day {}...", yd.year, yd.day);

        let destination = self.destination.unwrap_or_else(|| {
            let mut path = PathBuf::new();
            path.push("solutions");
            path.push("src");
            path.push("bin");
            path.push(format!("year{}-day{:02}.rs", yd.year, yd.day));
            path
        });

        if destination.exists() && !self.force {
            let mut error = Cli::command().error(
                ErrorKind::ValueValidation,
                format!(
                    "a solution already exists for year {}, day {}: {}. Did you mean to use --force or provide a date?",
                    yd.year,
                    yd.day,
                    destination.display()
                ),
            );

            // NOTE: context isn't printed to the user by default, so it's also included i nthe error message above
            error.insert(
                ContextKind::SuggestedArg,
                ContextValue::String("use --overwrite to overwrite the existing file".into()),
            );

            return Err(error);
        }

        let contents = Self::contents(&yd);
        std::fs::write(&destination, contents)
            .map_err(|e| Cli::command().error(ErrorKind::Io, e))?;

        Ok(ExitCode::SUCCESS)
    }
}

impl Cli {
    pub fn run(self) -> ExitCode {
        let command = self.command.unwrap_or_default();
        match command {
            Command::Run(args) => args.run(),
            Command::Daily(args) => args.run(),
        }
        .unwrap_or_else(|e| e.exit())
    }
}

use super::year_day;
use crate::{cli::exit_code_from_status, types::YearDay, Cli};
use clap::{error::ErrorKind, CommandFactory};
use std::{
    fs::File,
    path::PathBuf,
    process::{ExitCode, Stdio},
};

#[derive(clap::Args, Default)]
pub struct Args {
    #[clap(flatten)]
    yd: year_day::Args,

    /// Whether to run the solution in debug mode.
    #[clap(short, long, default_value_t = false)]
    debug: bool,
}

impl Args {
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

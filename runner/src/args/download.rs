use super::year_day;
use crate::{cli::exit_code_from_status, Cli};
use clap::{error::ErrorKind, CommandFactory};
use std::{
    path::PathBuf,
    process::{Command, ExitCode},
};

#[derive(clap::Args, Default)]
pub struct Args {
    #[clap(flatten)]
    yd: year_day::Args,

    /// The file to write the output to.
    ///
    /// If not provided, the destination will be automatically derived from the year and day.
    destination: Option<PathBuf>,
}

impl Args {
    pub fn run(self) -> Result<ExitCode, clap::Error> {
        let yd = self.yd.validate()?;
        eprintln!("Downloading for year {}, day {}...", yd.year, yd.day);

        let destination = if let Some(destination) = self.destination {
            destination
        } else {
            let mut path = PathBuf::new();
            path.push("solutions");
            path.push("data");
            path.push("real");
            path.push(format!("{:04}", yd.year));

            std::fs::create_dir_all(&path).map_err(|e| Cli::command().error(ErrorKind::Io, e))?;

            path.push(format!("{:02}.txt", yd.day));
            path
        };

        Command::new("aoc")
            .args([
                "download",
                "--overwrite",
                "--input-only",
                "--input-file",
                destination.to_str().unwrap(),
            ])
            .status()
            .map(exit_code_from_status)
            .map_err(|e| Cli::command().error(ErrorKind::Io, e))?;

        Ok(ExitCode::SUCCESS)
    }
}

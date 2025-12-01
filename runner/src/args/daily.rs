use clap::{
    error::{ContextKind, ContextValue, ErrorKind},
    CommandFactory,
};

use super::year_day;
use crate::{types::YearDay, Cli};
use std::{path::PathBuf, process::ExitCode};

#[derive(clap::Args, Default)]
pub struct Args {
    #[clap(flatten)]
    yd: year_day::Args,

    /// The file to write the output to.
    ///
    /// If not provided, the destination will be automatically derived from the year and day.
    destination: Option<PathBuf>,

    /// Whether to overwrite the output file if it already exists.
    #[clap(short, long, default_value_t = false)]
    force: bool,
}

impl Args {
    fn contents(yd: &YearDay) -> String {
        let doc = format!("Day {}", yd.day);
        let year = yd.year.0;
        let day = yd.day.0;
        let example_input_include_str =
            format!("../../data/examples/{}/{:02}/1.txt", yd.year, yd.day);

        format!(
            r#"//! {doc}
//!
//! https://adventofcode.com/{year}/day/{day}

use tracing::instrument;

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {{
    0
}}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {{
    0
}}

fn main() {{
    solutions::main(part1, part2);
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

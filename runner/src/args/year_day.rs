use crate::{
    is_aoc_event_for,
    types::{Day, Year, YearDay},
    Cli,
};
use clap::{error::ErrorKind, CommandFactory};

#[derive(clap::Args, Default)]
pub struct Args {
    /// The year of the challenge.
    ///
    /// This is required if the current date is not during the Advent of Code event.
    year: Option<Year>,

    /// The day of the challenge.
    ///
    /// This is required if the current date is not during the Advent of Code event.
    day: Option<Day>,
}

impl Args {
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

    pub fn validate(&self) -> Result<YearDay, clap::Error> {
        let year = self
            .year()
            .map_err(|e| Cli::command().error(ErrorKind::MissingRequiredArgument, e))?;

        let day = self
            .day()
            .map_err(|e| Cli::command().error(ErrorKind::MissingRequiredArgument, e))?;

        Ok(YearDay { year, day })
    }
}

use crate::{
    types::{Day, Year, YearDay},
    Cli,
};
use clap::{error::ErrorKind, CommandFactory};

#[derive(clap::Args, Default)]
// Clap will give an "Argument group name must be unique" panic if two structs named `Args` are used in a single `#[derive(clap::Args)]`
// so we need to assign an explicit unique ID to this group.
#[group(id = module_path!())]
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
        self.year.or_else(Year::now_if_during_event).ok_or(
            "the year is required if the current date is not during the Advent of Code event",
        )
    }

    pub fn day(&self) -> Result<Day, &str> {
        self.day
            .or_else(Day::now_if_during_event)
            .ok_or("the day is required if the current date is not during the Advent of Code event")
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

use clap::Parser;

use crate::is_aoc_event_for;
use crate::types::{Day, Year};

/// Run an Advent of Code solution.
#[derive(Parser)]
pub struct Args {
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
    pub debug: bool,
}

impl Args {
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
}

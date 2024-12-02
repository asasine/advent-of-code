use chrono::Datelike;

mod args;
mod runner;
mod types;

pub use args::Args;
pub use runner::Runner;

/// Returns `true` if `dt` is during the Advent of Code event.
pub fn is_aoc_event_for(dt: impl Datelike) -> bool {
    dt.month() == 12 && dt.day() <= 25
}

use chrono::Datelike;

mod cli;
mod types;

pub use cli::Cli;

/// Returns `true` if `dt` is during the Advent of Code event.
pub fn is_aoc_event_for(dt: impl Datelike) -> bool {
    dt.month() == 12 && dt.day() <= 25
}

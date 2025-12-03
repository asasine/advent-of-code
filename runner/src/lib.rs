use chrono::Datelike;

mod args;
mod cli;
mod types;

pub use cli::Cli;

/// Returns `true` if `dt` is during the Advent of Code event.
pub fn is_aoc_event_for(dt: impl Datelike) -> bool {
    dt.month() == 12 && dt.day() <= (if dt.year() < 2025 { 25 } else { 12 })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_december_1_is_aoc_event() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
        assert!(is_aoc_event_for(date));
    }

    #[test]
    fn test_december_25_is_aoc_event() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
        assert!(is_aoc_event_for(date));
    }

    #[test]
    fn test_december_26_is_not_aoc_event() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 26).unwrap();
        assert!(!is_aoc_event_for(date));
    }

    #[test]
    fn test_november_is_not_aoc_event() {
        let date = NaiveDate::from_ymd_opt(2024, 11, 15).unwrap();
        assert!(!is_aoc_event_for(date));
    }

    #[test]
    fn test_january_is_not_aoc_event() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        assert!(!is_aoc_event_for(date));
    }

    #[test]
    fn test_2025_only_12_events() {
        let date = NaiveDate::from_ymd_opt(2025, 12, 13).unwrap();
        assert!(!is_aoc_event_for(date));
    }
}

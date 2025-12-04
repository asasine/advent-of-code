use chrono::Datelike;

/// Returns `true` if `dt` is during the Advent of Code event.
pub fn is_aoc_event_for(dt: &impl Datelike) -> bool {
    dt.month() == 12 && dt.day() as u8 <= last_event_day_for(dt.year() as u16)
}

/// Returns the last day of the Advent of Code event for the specified year.
pub fn last_event_day_for(year: u16) -> u8 {
    if year < 2025 {
        25
    } else {
        12
    }
}

/// Get the current datetime in the US Eastern time zone.
///
/// The Advent of Code event releases puzzles based on Eastern Time. Using this timezone helps ensure date and time
/// calculations align with the event's schedule.
///
/// See also: [`chrono_tz::US::Eastern`].
pub fn now() -> chrono::DateTime<chrono_tz::Tz> {
    chrono::Utc::now().with_timezone(&chrono_tz::US::Eastern)
}

/// Returns the current datetime in the US Eastern time zone if during the Advent of Code event, otherwise returns
/// [`None`].
pub fn now_if_during_event() -> Option<chrono::DateTime<chrono_tz::Tz>> {
    Some(now()).filter(is_aoc_event_for)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_december_1_is_aoc_event() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
        assert!(is_aoc_event_for(&date));
    }

    #[test]
    fn test_december_25_is_aoc_event() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
        assert!(is_aoc_event_for(&date));
    }

    #[test]
    fn test_december_26_is_not_aoc_event() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 26).unwrap();
        assert!(!is_aoc_event_for(&date));
    }

    #[test]
    fn test_november_is_not_aoc_event() {
        let date = NaiveDate::from_ymd_opt(2024, 11, 15).unwrap();
        assert!(!is_aoc_event_for(&date));
    }

    #[test]
    fn test_january_is_not_aoc_event() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        assert!(!is_aoc_event_for(&date));
    }

    #[test]
    fn test_2025_only_12_events() {
        let date = NaiveDate::from_ymd_opt(2025, 12, 13).unwrap();
        assert!(!is_aoc_event_for(&date));
    }
}

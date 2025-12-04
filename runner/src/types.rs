use core::fmt;
use std::str::FromStr;

use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, TimeZone};

use crate::datetime::{is_aoc_event_for, last_event_day_for, now, now_if_during_event};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Year(pub u16);

impl Year {
    /// Returns the current year if during the Advent of Code event, otherwise returns [`None`].
    pub fn now_if_during_event() -> Option<Self> {
        now_if_during_event().map(Self::from)
    }
}

impl Default for Year {
    /// Returns the current year if in December in the US Eastern time zone, otherwise returns the previous year.
    fn default() -> Self {
        let now = now();
        let year = now.year();
        if is_aoc_event_for(&now) {
            Self(year as u16)
        } else {
            Self(year as u16 - 1)
        }
    }
}

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Year {
    type Err = <u16 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u16::from_str(s).map(Self)
    }
}

impl From<NaiveDate> for Year {
    fn from(d: NaiveDate) -> Self {
        Self(d.year() as u16)
    }
}

impl From<NaiveDateTime> for Year {
    fn from(dt: NaiveDateTime) -> Self {
        dt.date().into()
    }
}

impl<T: TimeZone> From<DateTime<T>> for Year {
    fn from(dt: DateTime<T>) -> Self {
        Self(dt.year() as u16)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Day(pub u8);

impl Day {
    /// Returns the current day if during the Advent of Code event, otherwise returns [`None`].
    pub fn now_if_during_event() -> Option<Self> {
        now_if_during_event().map(Self::from)
    }
}

impl Default for Day {
    /// Returns the current day if in December in the US Eastern time zone, otherwise returns the last day of the event
    /// for the current year.
    ///
    /// See [`Year::default`] for details on how the default year is determined.
    fn default() -> Self {
        let now = now();
        let day = now.day();
        if is_aoc_event_for(&now) {
            Self(day as u8)
        } else {
            Self(last_event_day_for(now.year() as u16))
        }
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Day {
    type Err = <u8 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u8::from_str(s).map(Self)
    }
}

impl From<NaiveDate> for Day {
    fn from(d: NaiveDate) -> Self {
        Self(d.day() as u8)
    }
}

impl From<NaiveDateTime> for Day {
    fn from(dt: NaiveDateTime) -> Self {
        dt.date().into()
    }
}

impl<T: TimeZone> From<DateTime<T>> for Day {
    fn from(dt: DateTime<T>) -> Self {
        Self(dt.day() as u8)
    }
}

pub struct YearDay {
    pub year: Year,
    pub day: Day,
}

impl From<NaiveDate> for YearDay {
    fn from(d: NaiveDate) -> Self {
        Self {
            year: d.into(),
            day: d.into(),
        }
    }
}

impl From<NaiveDateTime> for YearDay {
    fn from(dt: NaiveDateTime) -> Self {
        Self {
            year: dt.into(),
            day: dt.into(),
        }
    }
}

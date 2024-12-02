use core::fmt;
use std::str::FromStr;

use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, TimeZone};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Year(pub u16);

impl Default for Year {
    fn default() -> Self {
        let now = chrono::Local::now();
        let year = now.year();
        if now.month() == 12 {
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

impl Default for Day {
    fn default() -> Self {
        let now = chrono::Local::now();
        let day = now.day();
        if now.month() == 12 {
            Self(day as u8)
        } else {
            Self(25)
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

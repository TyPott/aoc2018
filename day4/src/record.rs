use chrono::naive::NaiveDateTime;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

use super::error;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Record {
    pub timestamp: NaiveDateTime,
    pub event: Event,
}

impl FromStr for Record {
    type Err = error::ParseRecordError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref RECORD_REGEX: Regex =
                Regex::new(r"\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] ([\w #]+)")
                    .expect("Failed to compile Record regex");
        }

        let c = RECORD_REGEX
            .captures(s)
            .ok_or(error::ParseRecordError::RecordFormatError)?;

        Ok(Record {
            timestamp: NaiveDateTime::parse_from_str(&c[1], "%Y-%m-%d %H:%M")?,
            event: Event::from_str(&c[2])?,
        })
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Event {
    StartShift(usize),
    FallAsleep,
    WakeUp,
}

impl FromStr for Event {
    type Err = error::ParseRecordError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref EVENT_REGEX: Regex =
                Regex::new(r"(\d+)").expect("Failed to compile Event regex");
        }

        Ok(match s {
            "wakes up" => Event::WakeUp,
            "falls asleep" => Event::FallAsleep,
            s => Event::StartShift(
                EVENT_REGEX
                    .captures(s)
                    .ok_or(error::ParseRecordError::RecordFormatError)?[1]
                    .parse()?,
            ),
        })
    }
}

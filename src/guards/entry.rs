use std::{fmt, error};
use std::str::FromStr;
use chrono::prelude::*;
use super::State;

#[derive(Debug)]
pub struct ParseEntryError;

impl fmt::Display for ParseEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid input string")
    }
}

impl error::Error for ParseEntryError {
    fn description(&self) -> &str {
        "invalid input string"
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Entry {
    pub timestamp: DateTime<Utc>,
    pub state: State,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}


impl FromStr for Entry {
    type Err = ParseEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let timestamp = match Utc.datetime_from_str(&s[1..17], "%Y-%m-%d %H:%M") {
            Ok(value) => value,
            Err(_) => return Err(ParseEntryError),
        };
        let state: State = match s[19..].parse() {
            Ok(value) => value,
            Err(_) => return Err(ParseEntryError),
        };
        Ok(Entry { timestamp, state })
    }
}

#[test]
fn entry_parsing() {
    assert_eq!(
        Entry {
            state: State::Begin(10),
            timestamp: Utc.ymd(1518, 11, 01).and_hms(0, 0, 0)
        },
        "[1518-11-01 00:00] Guard #10 begins shift"
            .parse::<Entry>()
            .unwrap()
    );
    assert_eq!(
        Entry {
            state: State::Asleep,
            timestamp: Utc.ymd(1518, 11, 01).and_hms(0, 5, 0)
        },
        "[1518-11-01 00:05] falls asleep".parse::<Entry>().unwrap()
    );
    assert_eq!(
        Entry {
            state: State::Wake,
            timestamp: Utc.ymd(1518, 11, 01).and_hms(0, 25, 0)
        },
        "[1518-11-01 00:25] wakes up".parse::<Entry>().unwrap()
    );
    // assert_eq!(Entry{state: State::Begin(10), timestamp: Utc.ymd(1518, 11, 01).and_hms(0, 0, 0)}, "[1518-11-01 00:00] Guard #10 begins shift".parse::<Entry>().unwrap());
    // assert_eq!(Entry{state: State::Begin(10), timestamp: Utc.ymd(1518, 11, 01).and_hms(0, 0, 0)}, "[1518-11-01 00:00] Guard #10 begins shift".parse::<Entry>().unwrap());
}

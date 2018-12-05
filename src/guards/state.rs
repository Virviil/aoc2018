use std::error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ParseStateError;

#[derive(PartialEq, PartialOrd, Debug, Eq, Ord)]
pub enum State {
    Begin(u32),
    Asleep,
    Wake,
}

impl fmt::Display for ParseStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid input string")
    }
}

impl error::Error for ParseStateError {
    fn description(&self) -> &str {
        "invalid input string"
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl FromStr for State {
    type Err = ParseStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..1] {
            "G" => Ok(State::Begin(
                s.chars()
                    .skip(7)
                    .take(s.chars().count() - 7 - 13)
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap(),
            )),
            "w" => Ok(State::Wake),
            "f" => Ok(State::Asleep),
            _ => Err(ParseStateError),
        }
    }
}

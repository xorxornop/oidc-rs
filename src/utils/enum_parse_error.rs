use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumParseError {
    Empty,
    NoMatch(String)
}

impl EnumParseError {
}

impl Error for EnumParseError {
    fn description(&self) -> &str {
        match *self {
            EnumParseError::Empty => "input was empty, and couldn't be matched to an enum variant",
            EnumParseError::NoMatch(_) => "input could not be matched to any of the enum variants"
        }
    }
}

impl fmt::Display for EnumParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnumParseError::Empty => write!(f, "input was empty, and had no match to an enum variant"),
            EnumParseError::NoMatch(ref s) => write!(f, "{} had no match to any enum variant", s)
        }
    }
}
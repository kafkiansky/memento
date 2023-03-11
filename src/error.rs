use std::fmt::{Display, Formatter, Result};
use std::io::Error;
use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq)]
pub enum MementoError {
    TooLongKey(String),
    ConnectionReset,
    InvalidItem(String),
    IoError(String),
    InvalidIntegerValue(String),
}

impl Display for MementoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::TooLongKey(key) => write!(f, "key {key} is too long"),
            Self::ConnectionReset => write!(f, "connection reset by peer"),
            Self::InvalidItem(item) => write!(f, "cannot parse item {item}"),
            Self::IoError(err) => write!(f, "{}", err),
            Self::InvalidIntegerValue(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for MementoError {}

impl From<Error> for MementoError {
    fn from(value: Error) -> Self {
        Self::IoError(value.to_string())
    }
}

impl From<ParseIntError> for MementoError {
    fn from(value: ParseIntError) -> Self {
        Self::InvalidIntegerValue(value.to_string())
    }
}

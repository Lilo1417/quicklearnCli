use core::fmt;
use std::fmt::Display;

use rustyline::error::ReadlineError;

pub type Result<T> = std::result::Result<T, CoreError>;

#[derive(Debug)]
pub enum CoreError {
    Storage(String),
    Learning(String),
    Read(ReadlineError),
}

impl From<rusqlite::Error> for CoreError {
    fn from(error: rusqlite::Error) -> Self {
        CoreError::Storage(error.to_string())
    }
}
impl From<ReadlineError> for CoreError {
    fn from(error: ReadlineError) -> Self {
        CoreError::Read(error)
    }
}

impl Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::Storage(str) => write!(f, "{}", str),
            CoreError::Learning(str) => write!(f, "{}", str),
            CoreError::Read(err) => write!(f, "{}", err.to_string()),
        }
    }
}

impl From<std::io::Error> for CoreError {
    fn from(error: std::io::Error) -> Self {
        CoreError::Storage(error.to_string())
    }
}

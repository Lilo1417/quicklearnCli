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

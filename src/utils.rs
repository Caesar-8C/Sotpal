use std::fmt;
use Error::*;
use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
	General(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			General(s) => write!(f, "Error::General: {}", s),
		}
	}
}

impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Self { General(e.to_string()) }
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        General(e.to_string())
    }
}
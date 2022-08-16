use std::fmt;

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
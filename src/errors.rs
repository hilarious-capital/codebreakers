use std::fmt;

#[derive(Debug)]
pub enum Error {
    AsciiUppercaseError(String),
    EncipheringError(String),
    DecipheringError(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::EncipheringError(s) => write!(f, "Error: {}", s),
            Error::DecipheringError(s) => write!(f, "Error: {}", s),
            Error::AsciiUppercaseError(s) => write!(f, "Error: {}", s),
        }
    }
}


use std::fmt::{Display, Formatter};
use std::num::ParseFloatError;
use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum CacheError {
    GuidNotFound,
    ParseError,
    IllegalState {},
}

impl Display for CacheError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            CacheError::GuidNotFound => write!(formatter, "Guid not found"),
            CacheError::ParseError => write!(formatter, "Parse error"),
            CacheError::IllegalState {} => write!(formatter, "Illegal state"),
        }
    }
}

impl error::Error for CacheError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            CacheError::GuidNotFound => None,
            CacheError::ParseError => None,
            CacheError::IllegalState {} => None,
        }
    }
}

// implementing the From trait allows us to use the ? operator
impl From<ParseFloatError> for CacheError {
    fn from(_: ParseFloatError) -> Self {
        CacheError::ParseError
    }
}

// implementing the From trait allows us to use the ? operator
impl From<chrono::ParseError> for CacheError {
    fn from(_: chrono::ParseError) -> Self {
        CacheError::ParseError
    }
}

use std::fmt::{Display, Formatter};
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

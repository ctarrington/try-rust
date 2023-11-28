use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseFloatError;
use std::str::ParseBoolError;
use std::{error, fmt};

#[derive(Debug)]
pub enum CacheError {
    GuidNotFound,
    ParseError(Box<dyn Error>),
    IllegalState {},
}

impl PartialEq for CacheError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CacheError::GuidNotFound, CacheError::GuidNotFound) => true,
            (CacheError::IllegalState {}, CacheError::IllegalState {}) => true,
            (CacheError::ParseError(left_error), CacheError::ParseError(right_error)) => {
                let left_error = left_error.to_string();
                let right_error = right_error.to_string();
                left_error == right_error
            }
            _ => false,
        }
    }
}

impl Display for CacheError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            CacheError::GuidNotFound => write!(formatter, "Guid not found"),
            CacheError::ParseError(error) => {
                write!(formatter, "ParseError: {}", error.to_string().as_str())
            }
            CacheError::IllegalState {} => write!(formatter, "Illegal state"),
        }
    }
}

impl error::Error for CacheError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            CacheError::GuidNotFound => None,
            CacheError::ParseError(error) => Some(error.as_ref()),
            CacheError::IllegalState {} => None,
        }
    }
}

// implementing the From trait allows us to use the ? operator
impl From<ParseFloatError> for CacheError {
    fn from(parse_float_error: ParseFloatError) -> Self {
        CacheError::ParseError(Box::new(parse_float_error))
    }
}

// implementing the From trait allows us to use the ? operator
impl From<chrono::ParseError> for CacheError {
    fn from(chrono_parse_errror: chrono::ParseError) -> Self {
        CacheError::ParseError(Box::new(chrono_parse_errror))
    }
}

impl From<ParseBoolError> for CacheError {
    fn from(parse_bool_error: ParseBoolError) -> Self {
        CacheError::ParseError(Box::new(parse_bool_error))
    }
}

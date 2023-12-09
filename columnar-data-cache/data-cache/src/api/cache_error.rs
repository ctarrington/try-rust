use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseFloatError;
use std::str::ParseBoolError;
use uuid::Uuid;

#[derive(Debug)]
pub enum CacheError {
    GuidNotFound(Uuid),
    ParseError(Box<dyn Error>),
    IllegalState,
    DuplicateColumn(String),
}

impl PartialEq for CacheError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CacheError::GuidNotFound(left_guid), CacheError::GuidNotFound(right_guid)) => {
                left_guid == right_guid
            }
            (CacheError::IllegalState, CacheError::IllegalState) => true,
            (CacheError::DuplicateColumn(left_name), CacheError::DuplicateColumn(right_name)) => {
                left_name == right_name
            }
            (CacheError::ParseError(left_error), CacheError::ParseError(right_error)) => {
                left_error.to_string() == right_error.to_string()
            }
            _ => false,
        }
    }
}

impl Display for CacheError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            CacheError::GuidNotFound(guid) => write!(formatter, "Guid not found: {}", guid),
            CacheError::DuplicateColumn(name) => {
                write!(formatter, "Duplicate column: {}", name)
            }
            CacheError::ParseError(error) => {
                write!(formatter, "ParseError: {}", error.to_string().as_str())
            }
            CacheError::IllegalState {} => write!(formatter, "Illegal state"),
        }
    }
}

impl Error for CacheError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CacheError::GuidNotFound(..) => None,
            CacheError::DuplicateColumn(..) => None,
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
    fn from(parse_error: chrono::ParseError) -> Self {
        CacheError::ParseError(Box::new(parse_error))
    }
}

impl From<ParseBoolError> for CacheError {
    fn from(parse_bool_error: ParseBoolError) -> Self {
        CacheError::ParseError(Box::new(parse_bool_error))
    }
}

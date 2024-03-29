use crate::api::cache_error::CacheError;
use chrono::NaiveDateTime;

/// The parsers module contains type specific functions that parse a string value into a specific type.
/// Each function returns a Result<Option<T>, CacheError::ParseError> where T is the desired output type.

// This is a helper function that returns the best value based on the value and the default value.
fn get_value(value: &str, default_value: &str) -> Option<String> {
    if !value.is_empty() {
        Some(value.trim().to_string())
    } else if !default_value.is_empty() {
        Some(default_value.trim().to_string())
    } else {
        None
    }
}

/// Parses a string value into a boolean. The following values are supported:
/// * 0, 1, true, false with case insensitivity
pub fn parse_bool(value: &str, default_value: &str) -> Result<Option<bool>, CacheError> {
    match get_value(value, default_value) {
        Some(the_value) => match the_value.as_str().to_ascii_lowercase().trim() {
            "0" => Ok(Some(false)),
            "1" => Ok(Some(true)),
            "true" => Ok(Some(true)),
            "false" => Ok(Some(false)),
            _ => Ok(Some(the_value.parse::<bool>()?)),
        },
        _ => Ok(None),
    }
}

/// Parses a string value into a f64.
pub fn parse_f64(value: &str, default_value: &str) -> Result<Option<f64>, CacheError> {
    match get_value(value, default_value) {
        Some(the_value) => Ok(Some(the_value.parse::<f64>()?)),
        _ => Ok(None),
    }
}

pub fn parse_string(value: &str, default_value: &str) -> Result<Option<String>, CacheError> {
    match get_value(value, default_value) {
        Some(the_value) => Ok(Some(the_value)),
        _ => Ok(None),
    }
}

pub fn parse_date_time(
    value: &str,
    default_value: &str,
    format: &str,
) -> Result<Option<NaiveDateTime>, CacheError> {
    let wrapped_value = get_value(value, default_value);
    match wrapped_value {
        Some(value) => Ok(Some(NaiveDateTime::parse_from_str(value.as_str(), format)?)),
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_boolean() {
        assert_eq!(parse_bool("true", "").unwrap(), Some(true));
        assert_eq!(parse_bool("TRUE", "false").unwrap(), Some(true));
        assert_eq!(parse_bool("1", "").unwrap(), Some(true));

        assert_eq!(parse_bool("false", "").unwrap(), Some(false));
        assert_eq!(parse_bool("FALSE", "").unwrap(), Some(false));
        assert_eq!(parse_bool("0", "").unwrap(), Some(false));

        assert_eq!(parse_bool("", "true").unwrap(), Some(true));
        assert_eq!(parse_bool("", "false").unwrap(), Some(false));
        assert_eq!(parse_bool("", "").unwrap(), None);
    }

    #[test]
    fn test_boolean_errors() {
        if let Err(error) = parse_bool("invalid", "true") {
            assert_eq!(
                error.to_string(),
                "ParseError: provided string was not `true` or `false`"
            );
            assert_eq!(
                error.source().unwrap().to_string(),
                "provided string was not `true` or `false`"
            );
        } else {
            panic!("Expected ParseError");
        }
    }

    #[test]
    fn test_f64() {
        assert_eq!(parse_f64("1.0", "").unwrap(), Some(1.0));
        assert_eq!(parse_f64("1.0", "2.0").unwrap(), Some(1.0));
        assert_eq!(parse_f64("", "1").unwrap(), Some(1.0));
        assert_eq!(parse_f64("", " 1").unwrap(), Some(1.0));
        assert!(parse_f64("invalid", "1").is_err());
        assert!(parse_f64("", "invalid").is_err());
    }

    #[test]
    fn test_f64_errors() {
        if let Err(error) = parse_f64("invalid", "") {
            assert_eq!(error.to_string(), "ParseError: invalid float literal");
            assert_eq!(error.source().unwrap().to_string(), "invalid float literal");
        } else {
            panic!("Expected ParseError");
        }
    }

    #[test]
    fn test_date_time() {
        const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

        assert_eq!(
            parse_date_time("2020-01-01 00:00:00", "", DATE_TIME_FORMAT).unwrap(),
            Some(NaiveDateTime::parse_from_str("2020-01-01 00:00:00", DATE_TIME_FORMAT).unwrap())
        );

        assert_eq!(
            parse_date_time(
                "2020-01-01 00:00:00",
                "2022-01-01 00:00:00",
                DATE_TIME_FORMAT
            )
            .unwrap(),
            Some(NaiveDateTime::parse_from_str("2020-01-01 00:00:00", DATE_TIME_FORMAT).unwrap())
        );

        assert_eq!(
            parse_date_time("", "2022-01-01 00:00:00", DATE_TIME_FORMAT).unwrap(),
            Some(NaiveDateTime::parse_from_str("2022-01-01 00:00:00", DATE_TIME_FORMAT).unwrap())
        );
    }

    #[test]
    fn test_date_time_errors() {
        const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

        if let Err(error) = parse_date_time("invalid", "", DATE_TIME_FORMAT) {
            assert_eq!(
                error.to_string(),
                "ParseError: input contains invalid characters"
            );
            assert_eq!(
                error.source().unwrap().to_string(),
                "input contains invalid characters"
            );
        } else {
            panic!("Expected ParseError");
        }
    }
}

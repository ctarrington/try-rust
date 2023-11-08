// This is a helper function that returns the value of a column based on the passed value and the
// default value for the column definition.
fn get_value(value: &str, default_value: &str) -> Option<String> {
    if !value.is_empty() {
        Some(value.trim().to_string())
    } else if !default_value.is_empty() {
        Some(default_value.trim().to_string())
    } else {
        None
    }
}

#[derive(Debug, PartialEq)]
pub struct TypeParseError {}

pub fn parse_bool(value: &str, default_value: &str) -> Result<Option<bool>, TypeParseError> {
    match get_value(value, default_value) {
        Some(the_value) => match the_value.as_str().to_ascii_lowercase().trim() {
            "true" => Ok(Some(true)),
            "false" => Ok(Some(false)),
            "0" => Ok(Some(false)),
            "1" => Ok(Some(true)),
            _ => Err(TypeParseError {}),
        },
        _ => Ok(None),
    }
}

pub fn parse_f64(value: &str, defualt_value: &str) -> Result<Option<f64>, TypeParseError> {
    match get_value(value, defualt_value) {
        Some(the_value) => the_value
            .parse::<f64>()
            .map(Some)
            .map_err(|_| TypeParseError {}),
        _ => Ok(None),
    }
}

pub fn parse_string(value: &str, default_value: &str) -> Result<Option<String>, TypeParseError> {
    match get_value(value, default_value) {
        Some(the_value) => Ok(Some(the_value)),
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean() {
        assert_eq!(parse_bool("true", "").unwrap(), Some(true));
        assert_eq!(parse_bool("TRUE", "false").unwrap(), Some(true));
        assert_eq!(parse_bool("false", "").unwrap(), Some(false));
        assert_eq!(parse_bool("FALSE", "").unwrap(), Some(false));
        assert_eq!(parse_bool("0", "").unwrap(), Some(false));
        assert_eq!(parse_bool("1", "").unwrap(), Some(true));

        assert_eq!(parse_bool("", "true").unwrap(), Some(true));
        assert_eq!(parse_bool("", "false").unwrap(), Some(false));

        assert!(parse_bool("invalid", "true").is_err());
        assert!(parse_bool("", "invalid").is_err());
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
}

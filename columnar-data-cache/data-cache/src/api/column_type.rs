use chrono::NaiveDateTime;

/// The structs in this file are used to define the columns for a data cache.
/// They allow client code to define the type of each column.
/// Also, the default value is used when the client code does not provide a value when parsing
/// a row of data. The details of the parsing are handled by the column types
///
/// There are a few subtleties to note:
/// 1. The DateTimeColumnType struct has a format string that is used to parse the date.
///    The passed data value must match the format string or an error result will be returned.
/// 2. The NumericColumnType struct expects a string value that can be parsed as a f64.
///    If the passed value does not parse,
/// 3. The NumericColumnType struct has a units field that can be used by client code. It does not
///    affect the parsing or storage of the value.

// This is a helper function that returns the value of a column based on the passed value and the
// default value for the column definition.
fn get_value(default_value: &str, value: &str) -> Option<String> {
    if !value.is_empty() {
        Some(value.to_string())
    } else if !default_value.is_empty() {
        Some(default_value.to_string())
    } else {
        None
    }
}

#[derive(Debug, PartialEq)]
pub struct TypeParseError {}

#[derive(Debug, PartialEq)]
pub struct StringColumnType {}

impl StringColumnType {
    pub fn parse(
        &self,
        default_value: &str,
        value: &str,
    ) -> Result<Option<String>, TypeParseError> {
        Ok(get_value(default_value, value))
    }
}

#[derive(Debug, PartialEq)]
pub struct BooleanColumnType {}

/// The BooleanColumnType struct implements the parse method by treating "true" and "1" as true
/// and "false" and "0" as false.
/// Blank is not false, but rather None.
impl BooleanColumnType {
    pub fn parse(&self, default_value: &str, value: &str) -> Result<Option<bool>, TypeParseError> {
        match get_value(default_value, value) {
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
}
#[derive(Debug, PartialEq)]
pub struct DateTimeColumnType {
    format: String,
}

/// The DateTimeColumnType struct implements the parse method by using the column definition's format string
/// Note that invalid dates will return an error result.
impl DateTimeColumnType {
    pub fn parse(
        &self,
        default_value: &str,
        value: &str,
    ) -> Result<Option<NaiveDateTime>, TypeParseError> {
        let the_value = get_value(default_value, value).unwrap();
        if the_value.is_empty() {
            return Ok(None);
        }

        NaiveDateTime::parse_from_str(the_value.as_str(), self.format.as_str())
            .map(Some)
            .map_err(|_| TypeParseError {})
    }
}
#[derive(Debug, PartialEq)]
pub struct NumericColumnType {
    units: String,
}

impl NumericColumnType {
    pub fn parse(&self, default_value: &str, value: &str) -> Result<Option<f64>, TypeParseError> {
        match get_value(default_value, value) {
            Some(the_value) => the_value
                .parse::<f64>()
                .map(Some)
                .map_err(|_| TypeParseError {}),
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    #[test]
    fn test_string_column_type() {
        let column_type = StringColumnType {};
        assert_eq!(column_type.parse("-", "").unwrap(), Some("-".to_string()));
    }

    #[test]
    fn test_boolean_column_type() {
        let boolean_column_type = BooleanColumnType {};
        assert_eq!(boolean_column_type.parse("", "true").unwrap(), Some(true));
        assert_eq!(boolean_column_type.parse("", "True").unwrap(), Some(true));
        assert_eq!(boolean_column_type.parse("", " True ").unwrap(), Some(true));
        assert_eq!(boolean_column_type.parse("", "1").unwrap(), Some(true));

        assert_eq!(boolean_column_type.parse("", "false").unwrap(), Some(false));
        assert_eq!(boolean_column_type.parse("", "FALSE").unwrap(), Some(false));
        assert_eq!(boolean_column_type.parse("", "0").unwrap(), Some(false));

        assert_eq!(boolean_column_type.parse("", "").unwrap(), None);
        assert!(boolean_column_type.parse("", "2").is_err());
        assert!(boolean_column_type.parse("", "True2").is_err());
    }

    #[test]
    fn test_numeric_column_type() {
        let column_type = NumericColumnType {
            units: "m/s".to_string(),
        };
        assert_eq!(column_type.parse("", "1.0"), Ok(Some(1.0)));
        assert_eq!(column_type.parse("", "1.123"), Ok(Some(1.123)));
        assert_eq!(column_type.parse("", "").unwrap(), None);
        assert!(column_type.parse("", "abc").is_err());
    }

    #[test]
    fn test_datetime_column_type() {
        let datetime_column_type = DateTimeColumnType {
            format: DATE_TIME_FORMAT.to_string(),
        };
        assert_eq!(
            datetime_column_type.parse("", "2020-01-01 00:00:00"),
            Ok(Some(
                NaiveDateTime::parse_from_str("2020-01-01 00:00:00", DATE_TIME_FORMAT).unwrap()
            ))
        );
        assert!(datetime_column_type.parse("", "qqq").is_err());
    }
}

use chrono::NaiveDateTime;

/// The structs in this file are used to define the columns for a data cache.
/// They allow client code to define the type of each column.
/// Also, the default value is used when the client code does not provide a value when parsing
/// a row of data. The details of the parsing are handled by the column types
///
/// There are a few subtleties to note:
/// 1. The column type is an enum, but the parse method is implemented on each of the enum variants.
/// 2. The DateTimeColumnType struct has a format string that is used to parse the date.
///    The passed data value must match the format string or a panic will occur.
/// 3. The NumericColumnType struct expects a string value that can be parsed as a f64.
///    If the passed value does not parse, a panic will occur.
/// 4. The NumericColumnType struct has a units field that can be used by client code. It does not
///    affect the parsing or storage of the value.

// This is a helper function that returns the value of a column based on the passed value and the
// default value for the column definition.

// todo: units should be an enum?

fn get_value(column_definition: &ColumnDefinition, value: &str) -> Option<String> {
    if !value.is_empty() {
        Some(value.to_string())
    } else if !column_definition.get_default_value().is_empty() {
        Some(column_definition.get_default_value().to_string())
    } else {
        None
    }
}

#[derive(Debug, PartialEq)]
pub struct StringColumnType {}

impl StringColumnType {
    pub fn parse(&self, column_definition: &ColumnDefinition, value: &str) -> Option<String> {
        get_value(column_definition, value)
    }
}

#[derive(Debug, PartialEq)]
pub struct BooleanColumnType {}

/// The BooleanColumnType struct implements the parse method by treating "true" and "1" as true
/// and "false" and "0" as false.
/// Blank is not false, but rather None.
impl BooleanColumnType {
    pub fn parse(&self, column_definition: &ColumnDefinition, value: &str) -> Option<bool> {
        match get_value(column_definition, value) {
            Some(the_value) => match the_value.as_str().to_ascii_lowercase().trim() {
                "true" => Some(true),
                "false" => Some(false),
                "0" => Some(false),
                "1" => Some(true),
                _ => None,
            },
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct DateTimeColumnType {
    format: String,
}

/// The DateTimeColumnType struct implements the parse method by using the column definition's format string
/// Note that invalid dates will cause a panic.
impl DateTimeColumnType {
    pub fn parse(
        &self,
        column_definition: &ColumnDefinition,
        value: &str,
    ) -> Option<NaiveDateTime> {
        let the_value = get_value(column_definition, value);
        match the_value {
            Some(the_value) => Some(
                NaiveDateTime::parse_from_str(the_value.as_str(), self.format.as_str()).unwrap(),
            ),
            None => None,
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct NumericColumnType {
    units: String,
}

impl NumericColumnType {
    pub fn parse(&self, column_definition: &ColumnDefinition, value: &str) -> Option<f64> {
        let the_value = get_value(column_definition, value);
        match the_value {
            Some(the_value) => Some(the_value.parse::<f64>().unwrap()),
            None => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ColumnType {
    StringColumnType(StringColumnType),
    NumericColumnType(NumericColumnType),
    BooleanColumnType(BooleanColumnType),
    DateTimeColumnType(DateTimeColumnType),
}

pub struct ColumnDefinition {
    name: String,
    view_name: String,
    column_type: ColumnType,
    default_value: String,
}

impl ColumnDefinition {
    pub fn new(name: String, view_name: String, default_value: String) -> Self {
        Self {
            name,
            view_name,
            column_type: ColumnType::StringColumnType(StringColumnType {}),
            default_value,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_view_name(&self) -> &str {
        &self.view_name
    }

    pub fn get_column_type(&self) -> &ColumnType {
        &self.column_type
    }

    pub fn get_default_value(&self) -> &str {
        &self.default_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    fn generate_string_column_definition() -> ColumnDefinition {
        ColumnDefinition {
            name: "name".parse().unwrap(),
            view_name: "The Name".to_string(),
            column_type: ColumnType::StringColumnType(StringColumnType {}),
            default_value: "-".to_string(),
        }
    }

    fn generate_boolean_column_definition() -> ColumnDefinition {
        ColumnDefinition {
            name: "ok".to_string(),
            view_name: "OK".to_string(),
            column_type: ColumnType::BooleanColumnType(BooleanColumnType {}),
            default_value: "".to_string(),
        }
    }

    fn generate_numeric_column_definition() -> ColumnDefinition {
        ColumnDefinition {
            name: "speed".parse().unwrap(),
            view_name: "Speed".to_string(),
            column_type: ColumnType::NumericColumnType(NumericColumnType {
                units: "meters/second".to_string(),
            }),
            default_value: "".to_string(),
        }
    }

    fn generate_datetime_column_definition() -> ColumnDefinition {
        ColumnDefinition {
            name: "startDate".parse().unwrap(),
            view_name: "Start Date".to_string(),
            column_type: ColumnType::DateTimeColumnType(DateTimeColumnType {
                format: DATE_TIME_FORMAT.to_string(),
            }),
            default_value: "".to_string(),
        }
    }

    #[test]
    fn test_string_column_type() {
        let string_column_type = StringColumnType {};
        let definition = generate_string_column_definition();
        assert_eq!(
            string_column_type.parse(&definition, "value"),
            Some("value".to_string())
        );
        assert_eq!(
            string_column_type.parse(&definition, ""),
            Some("-".to_string())
        );
    }

    #[test]
    fn test_boolean_column_type() {
        let boolean_column_type = BooleanColumnType {};
        let definition = generate_boolean_column_definition();
        assert_eq!(boolean_column_type.parse(&definition, "true"), Some(true));
        assert_eq!(boolean_column_type.parse(&definition, "True"), Some(true));
        assert_eq!(boolean_column_type.parse(&definition, " True "), Some(true));
        assert_eq!(boolean_column_type.parse(&definition, "1"), Some(true));

        assert_eq!(boolean_column_type.parse(&definition, "false"), Some(false));
        assert_eq!(boolean_column_type.parse(&definition, "FALSE"), Some(false));
        assert_eq!(boolean_column_type.parse(&definition, "0"), Some(false));

        assert_eq!(boolean_column_type.parse(&definition, ""), None);
        assert_eq!(boolean_column_type.parse(&definition, "2"), None);
        assert_eq!(boolean_column_type.parse(&definition, "True2"), None);
    }

    #[test]
    fn test_numeric_column_type() {
        let numeric_column_type = NumericColumnType {
            units: "meters/second".to_string(),
        };
        let definition = generate_numeric_column_definition();
        assert_eq!(numeric_column_type.parse(&definition, "1.0"), Some(1.0));
        assert_eq!(numeric_column_type.parse(&definition, "1.123"), Some(1.123));
        assert_eq!(numeric_column_type.parse(&definition, ""), None);
        assert_eq!(numeric_column_type.units, "meters/second".to_string());
    }

    #[test]
    fn test_datetime_column_type() {
        let datetime_column_type = DateTimeColumnType {
            format: DATE_TIME_FORMAT.to_string(),
        };
        let definition = generate_datetime_column_definition();
        assert_eq!(
            datetime_column_type.parse(&definition, "2020-01-01 00:00:00"),
            Some(NaiveDateTime::parse_from_str("2020-01-01 00:00:00", DATE_TIME_FORMAT).unwrap())
        );
    }

    #[test]
    fn test_string_column_definition() {
        let column_definition = generate_string_column_definition();
        assert_eq!(column_definition.get_name(), "name");
        assert_eq!(column_definition.get_view_name(), "The Name");
        assert_eq!(
            column_definition.get_column_type(),
            &ColumnType::StringColumnType(StringColumnType {})
        );
        assert_eq!(column_definition.get_default_value(), "-");
    }

    #[test]
    fn test_boolean_column_definition() {
        let column_definition = generate_boolean_column_definition();
        assert_eq!(column_definition.get_name(), "ok");
        assert_eq!(column_definition.get_view_name(), "OK");
        assert_eq!(
            column_definition.get_column_type(),
            &ColumnType::BooleanColumnType(BooleanColumnType {})
        );
        assert_eq!(column_definition.get_default_value(), "");
    }
}

use chrono::NaiveDateTime;

/// Column definition

// This is a helper function that returns the value of a column based on the passed value and the
// default value for the column definition.
fn get_value(column_definition: ColumnDefinition, value: &str) -> Option<String> {
    if value.len() > 0 {
        Some(value.to_string())
    } else if column_definition.get_default_value().len() > 0 {
        Some(column_definition.get_default_value().to_string())
    } else {
        None
    }
}

#[derive(Debug, PartialEq)]
pub struct StringColumnType {}

impl StringColumnType {
    pub fn parse(&self, column_definition: ColumnDefinition, value: &str) -> Option<String> {
        get_value(column_definition, value)
    }
}

#[derive(Debug, PartialEq)]
pub struct BooleanColumnType {}

/// The BooleanColumnType struct implements the parse method by treating "true" and "1" as true
/// and "false" and "0" as false.
/// Blank is not false, but rather None.
impl BooleanColumnType {
    pub fn parse(&self, column_definition: ColumnDefinition, value: &str) -> Option<bool> {
        match get_value(column_definition, value) {
            Some(the_value) => match the_value.as_str() {
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
        column_definition: ColumnDefinition,
        value: &str,
    ) -> Option<chrono::NaiveDateTime> {
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
    precision: u8,
}

impl NumericColumnType {
    pub fn parse(&self, column_definition: ColumnDefinition, value: &str) -> Option<f64> {
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
    units: String,
    column_type: ColumnType,
    default_value: String,
}

impl ColumnDefinition {
    pub fn new(name: String, view_name: String, units: String, default_value: String) -> Self {
        Self {
            name,
            view_name,
            units,
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

    pub fn get_units(&self) -> &str {
        &self.units
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

    fn generate_string_column_definition() -> ColumnDefinition {
        ColumnDefinition {
            name: "name".parse().unwrap(),
            view_name: "The Name".to_string(),
            units: "".to_string(),
            column_type: ColumnType::StringColumnType(StringColumnType {}),
            default_value: "-".to_string(),
        }
    }

    fn generate_boolean_column_definition() -> ColumnDefinition {
        ColumnDefinition {
            name: "ok".parse().unwrap(),
            view_name: "OK".to_string(),
            units: "".to_string(),
            column_type: ColumnType::BooleanColumnType(BooleanColumnType {}),
            default_value: "".to_string(),
        }
    }

    fn generate_numeric_column_definition() -> ColumnDefinition {
        ColumnDefinition {
            name: "speed".parse().unwrap(),
            view_name: "Speedd".to_string(),
            units: "meters/second".to_string(),
            column_type: ColumnType::NumericColumnType(NumericColumnType { precision: 2 }),
            default_value: "".to_string(),
        }
    }

    fn generate_datetime_column_definition() -> ColumnDefinition {
        ColumnDefinition {
            name: "startdate".parse().unwrap(),
            view_name: "Start Date".to_string(),
            units: "".to_string(),
            column_type: ColumnType::DateTimeColumnType(DateTimeColumnType {
                format: "%Y-%m-%d %H:%M:%S".to_string(),
            }),
            default_value: "".to_string(),
        }
    }

    #[test]
    fn test_string_column_type() {
        let string_column_type = StringColumnType {};
        assert_eq!(
            string_column_type.parse(generate_string_column_definition(), "value"),
            Some("value".to_string())
        );
    }

    #[test]
    fn test_boolean_column_type() {
        let boolean_column_type = BooleanColumnType {};
        assert_eq!(
            boolean_column_type.parse(generate_boolean_column_definition(), "true"),
            Some(true)
        );
    }

    #[test]
    fn test_numeric_column_type() {
        let numeric_column_type = NumericColumnType { precision: 2 };
        assert_eq!(
            numeric_column_type.parse(generate_numeric_column_definition(), "1.0"),
            Some(1.0)
        );
    }

    #[test]
    fn test_datetime_column_type() {
        let datetime_column_type = DateTimeColumnType {
            format: "%Y-%m-%d %H:%M:%S".to_string(),
        };
        assert_eq!(
            datetime_column_type
                .parse(generate_datetime_column_definition(), "2020-01-01 00:00:00"),
            Some(
                NaiveDateTime::parse_from_str("2020-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
            )
        );
    }

    #[test]
    fn test_string_column_definition() {
        let column_definition = generate_string_column_definition();
        assert_eq!(column_definition.get_name(), "name");
        assert_eq!(column_definition.get_view_name(), "The Name");
        assert_eq!(column_definition.get_units(), "");
        assert_eq!(
            column_definition.get_column_type(),
            &ColumnType::StringColumnType(StringColumnType {})
        );
        assert_eq!(column_definition.get_default_value(), "-");
    }
}

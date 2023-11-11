use crate::api::column::Column;
use crate::api::parsers::{parse_bool, parse_date_time, parse_f64, parse_string, TypeParseError};

pub enum ColumnStorage {
    StringStorage {
        column: Column,
        data: Vec<Option<String>>,
    },
    BooleanStorage {
        column: Column,
        data: Vec<Option<bool>>,
    },
    F64Storage {
        column: Column,
        data: Vec<Option<f64>>,
    },
    TimeDateStorage {
        column: Column,
        data: Vec<Option<chrono::NaiveDateTime>>,
        format: String,
    },
}

fn push_with_check<T>(
    data: &mut Vec<Option<T>>,
    parsed_value: Result<Option<T>, TypeParseError>,
) -> Result<Option<()>, TypeParseError> {
    return match parsed_value {
        Ok(value) => {
            data.push(value);
            Ok(None)
        }
        Err(_) => {
            data.push(None);
            Err(TypeParseError {})
        }
    };
}

impl ColumnStorage {
    pub fn add_value(&mut self, value: &str) -> Result<Option<()>, TypeParseError> {
        match self {
            ColumnStorage::BooleanStorage { data, column } => {
                let parsed_value = parse_bool(value, column.default_value.as_str());
                push_with_check(data, parsed_value)
            }
            ColumnStorage::F64Storage { data, column } => {
                let parsed_value = parse_f64(value, column.default_value.as_str());
                push_with_check(data, parsed_value)
            }
            ColumnStorage::StringStorage { data, column } => {
                let parsed_value = parse_string(value, column.default_value.as_str());
                push_with_check(data, parsed_value)
            }
            ColumnStorage::TimeDateStorage {
                data,
                column,
                format,
            } => {
                let parsed_value = parse_date_time(value, column.default_value.as_str(), format);
                push_with_check(data, parsed_value)
            }
        }
    }

    pub fn get_length(&self) -> usize {
        match self {
            ColumnStorage::BooleanStorage { data, .. } => data.len(),
            ColumnStorage::F64Storage { data, .. } => data.len(),
            ColumnStorage::StringStorage { data, .. } => data.len(),
            ColumnStorage::TimeDateStorage { data, .. } => data.len(),
        }
    }

    pub fn remove_last_value(&mut self) {
        match self {
            ColumnStorage::BooleanStorage { data, .. } => {
                data.pop();
            }
            ColumnStorage::F64Storage { data, .. } => {
                data.pop();
            }
            ColumnStorage::StringStorage { data, .. } => {
                data.pop();
            }
            ColumnStorage::TimeDateStorage { data, .. } => {
                data.pop();
            }
        }
    }

    pub fn get_as_string(&self, index: usize) -> String {
        match self {
            ColumnStorage::BooleanStorage { data, .. } => match data.get(index) {
                Some(value) => match value {
                    Some(value) => value.to_string(),
                    None => "".to_string(),
                },
                None => "".to_string(),
            },
            ColumnStorage::F64Storage { data, .. } => match data.get(index) {
                Some(value) => match value {
                    Some(value) => value.to_string(),
                    None => "".to_string(),
                },
                None => "".to_string(),
            },
            ColumnStorage::StringStorage { data, .. } => match data.get(index) {
                Some(value) => match value {
                    Some(value) => value.to_string(),
                    None => "".to_string(),
                },
                None => "".to_string(),
            },
            ColumnStorage::TimeDateStorage { data, .. } => match data.get(index) {
                Some(value) => match value {
                    Some(value) => value.to_string(),
                    None => "".to_string(),
                },
                None => "".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_storage() {
        let column = Column::new("verified", "Verified", "false");
        let mut storage = ColumnStorage::BooleanStorage {
            column,
            data: vec![],
        };

        let result = storage.add_value("true");
        assert_eq!(result, Ok(None));
        assert_eq!(storage.get_as_string(0), "true");
    }

    #[test]
    fn test_time_date_storage() {
        let column = Column::new("starttime", "Start Time", "");
        let mut storage = ColumnStorage::TimeDateStorage {
            column,
            data: vec![],
            format: "%Y-%m-%d %H:%M:%S".to_string(),
        };

        let result = storage.add_value("2020-01-01 00:00:00");
        assert_eq!(result, Ok(None));
        assert_eq!(storage.get_as_string(0), "2020-01-01 00:00:00");
    }
}

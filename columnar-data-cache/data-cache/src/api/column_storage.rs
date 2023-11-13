use crate::api::column::Column;
use crate::api::parsers::{parse_bool, parse_date_time, parse_f64, parse_string, TypeParseError};
#[derive(Debug, PartialEq)]
pub struct IndexRangeError {}

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
    match parsed_value {
        Ok(value) => {
            data.push(value);
            Ok(None)
        }
        Err(_) => {
            data.push(None);
            Err(TypeParseError {})
        }
    }
}

fn value_to_string<T>(data: &[Option<T>], index: usize) -> Result<String, IndexRangeError>
where
    T: ToString,
{
    if index >= data.len() {
        return Err(IndexRangeError {});
    }

    let wrapper = data.get(index);
    match wrapper {
        Some(Some(value)) => Ok(value.to_string()),
        Some(None) => Ok("".to_string()),
        None => Ok("".to_string()),
    }
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

    pub fn get_as_string(&self, index: usize) -> Result<String, IndexRangeError> {
        match self {
            ColumnStorage::BooleanStorage { data, .. } => value_to_string(data, index),
            ColumnStorage::F64Storage { data, .. } => value_to_string(data, index),
            ColumnStorage::StringStorage { data, .. } => value_to_string(data, index),
            ColumnStorage::TimeDateStorage { data, .. } => value_to_string(data, index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_boolean_storage(default_value: String) -> ColumnStorage {
        ColumnStorage::BooleanStorage {
            column: Column::new("verified", "Verified", default_value.as_str()),
            data: vec![],
        }
    }

    fn create_f64_storage(default_storage: String) -> ColumnStorage {
        ColumnStorage::F64Storage {
            column: Column::new("price", "Price", default_storage.as_str()),
            data: vec![],
        }
    }

    fn create_string_storage(default_value: String) -> ColumnStorage {
        ColumnStorage::StringStorage {
            column: Column::new("name", "Name", default_value.as_str()),
            data: vec![],
        }
    }

    fn create_time_date_storage() -> ColumnStorage {
        ColumnStorage::TimeDateStorage {
            column: Column::new("starttime", "Start Time", ""),
            data: vec![],
            format: "%Y-%m-%d %H:%M:%S".to_string(),
        }
    }

    #[test]
    fn test_string_storage() {
        let mut storage = create_string_storage("unknown".to_string());

        assert_eq!(storage.add_value("hello"), Ok(None));
        assert_eq!(storage.get_as_string(0), Ok("hello".to_string()));
        assert_eq!(storage.add_value(""), Ok(None));
        assert_eq!(storage.get_as_string(1), Ok("unknown".to_string()));

        let mut storage = create_string_storage("".to_string());
        assert_eq!(storage.add_value("hello"), Ok(None));
        assert_eq!(storage.get_as_string(0), Ok("hello".to_string()));
        assert_eq!(storage.add_value(""), Ok(None));
        assert_eq!(storage.get_as_string(1), Ok("".to_string()));
    }

    #[test]
    fn test_boolean_storage() {
        let mut storage = create_boolean_storage("false".to_string());

        assert_eq!(storage.add_value("true"), Ok(None));
        assert_eq!(storage.get_as_string(0), Ok("true".to_string()));

        assert_eq!(storage.add_value("tRue"), Ok(None));
        assert_eq!(storage.get_as_string(1), Ok("true".to_string()));

        assert_eq!(storage.add_value("xyz"), Err(TypeParseError {}));
        assert_eq!(storage.get_as_string(2), Ok("".to_string()));

        assert_eq!(storage.add_value(""), Ok(None));
        assert_eq!(storage.get_as_string(3), Ok("false".to_string()));

        let mut storage = create_boolean_storage("".to_string());
        assert_eq!(storage.add_value(""), Ok(None));
        assert_eq!(storage.get_as_string(0), Ok("".to_string()));

        assert_eq!(storage.add_value(" TRue "), Ok(None));
        assert_eq!(storage.get_as_string(1), Ok("true".to_string()));
    }

    #[test]
    fn test_f64_storage() {
        let mut storage = create_f64_storage("0".to_string());

        assert_eq!(storage.add_value("1.0"), Ok(None));
        assert_eq!(storage.get_as_string(0), Ok("1".to_string()));
        assert_eq!(storage.add_value(""), Ok(None));
        assert_eq!(storage.get_as_string(1), Ok("0".to_string()));
        assert_eq!(storage.add_value("xyz"), Err(TypeParseError {}));
        assert_eq!(storage.get_as_string(2), Ok("".to_string()));
        assert_eq!(storage.add_value("1.3"), Ok(None));
        assert_eq!(storage.get_as_string(3), Ok("1.3".to_string()));

        let mut storage = create_f64_storage("0".to_string());
        assert_eq!(storage.add_value(""), Ok(None));
        assert_eq!(storage.get_as_string(0), Ok("0".to_string()));
    }

    #[test]
    fn test_time_date_storage() {
        let mut storage = create_time_date_storage();

        assert_eq!(storage.add_value("2020-01-01 00:00:00"), Ok(None));
        assert_eq!(storage.add_value(""), Ok(None));
        assert_eq!(
            storage.get_as_string(0),
            Ok("2020-01-01 00:00:00".to_string())
        );
        assert_eq!(storage.get_as_string(1), Ok("".to_string()));
    }

    #[test]
    fn test_remove_last_value() {
        let mut storage = create_string_storage("unknown".to_string());
        storage.add_value("hello").unwrap();
        storage.remove_last_value();
        storage.remove_last_value();
    }
}

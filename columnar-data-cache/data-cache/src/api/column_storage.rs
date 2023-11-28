use crate::api::cache_error::CacheError;
use crate::api::column::Column;
use crate::api::parsers::{parse_bool, parse_date_time, parse_f64, parse_string};
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
    EnumeratedStorage {
        column: Column,
        data: Vec<Option<String>>,
        allowed_values: Vec<String>,
    },
}

fn push_with_check<T>(
    data: &mut Vec<Option<T>>,
    parsed_value: Result<Option<T>, CacheError>,
) -> Result<Option<()>, CacheError> {
    match parsed_value {
        Ok(value) => {
            data.push(value);
            Ok(None)
        }
        Err(error) => {
            data.push(None);
            Err(error)
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
    pub fn add_value(&mut self, value: &str) -> Result<Option<()>, CacheError> {
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
            ColumnStorage::EnumeratedStorage {
                data,
                column,
                allowed_values,
            } => {
                let parsed_value = parse_string(value, column.default_value.as_str());
                match parsed_value {
                    Ok(Some(value)) => {
                        if value.is_empty() {
                            data.push(None);
                            Ok(None)
                        } else if allowed_values.contains(&value) {
                            data.push(Some(value));
                            Ok(None)
                        } else {
                            data.push(None);
                            Err(CacheError::IllegalState {})
                        }
                    }
                    Ok(None) => {
                        data.push(None);
                        Ok(None)
                    }
                    Err(error) => {
                        data.push(None);
                        Err(error)
                    }
                }
            }
        }
    }

    pub fn get_length(&self) -> usize {
        match self {
            ColumnStorage::BooleanStorage { data, .. } => data.len(),
            ColumnStorage::F64Storage { data, .. } => data.len(),
            ColumnStorage::StringStorage { data, .. } => data.len(),
            ColumnStorage::TimeDateStorage { data, .. } => data.len(),
            ColumnStorage::EnumeratedStorage { data, .. } => data.len(),
        }
    }

    pub fn remove_value(&mut self, index: usize) -> Result<(), IndexRangeError> {
        match self {
            ColumnStorage::BooleanStorage { data, .. } => {
                data.remove(index);
                Ok(())
            }
            ColumnStorage::F64Storage { data, .. } => {
                data.remove(index);
                Ok(())
            }
            ColumnStorage::StringStorage { data, .. } => {
                data.remove(index);
                Ok(())
            }
            ColumnStorage::TimeDateStorage { data, .. } => {
                data.remove(index);
                Ok(())
            }
            ColumnStorage::EnumeratedStorage { data, .. } => {
                data.remove(index);
                Ok(())
            }
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
            ColumnStorage::EnumeratedStorage { data, .. } => {
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
            ColumnStorage::EnumeratedStorage { data, .. } => value_to_string(data, index),
        }
    }

    pub fn get_default_value(&self) -> String {
        match self {
            ColumnStorage::BooleanStorage { column, .. } => column.default_value.clone(),
            ColumnStorage::F64Storage { column, .. } => column.default_value.clone(),
            ColumnStorage::StringStorage { column, .. } => column.default_value.clone(),
            ColumnStorage::TimeDateStorage { column, .. } => column.default_value.clone(),
            ColumnStorage::EnumeratedStorage { column, .. } => column.default_value.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_boolean_storage(default_value: &str) -> ColumnStorage {
        ColumnStorage::BooleanStorage {
            column: Column::new("verified", "Verified", default_value),
            data: vec![],
        }
    }

    fn create_f64_storage(default_value: &str) -> ColumnStorage {
        ColumnStorage::F64Storage {
            column: Column::new("price", "Price", default_value),
            data: vec![],
        }
    }

    fn create_string_storage(default_value: &str) -> ColumnStorage {
        ColumnStorage::StringStorage {
            column: Column::new("name", "Name", default_value),
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

    fn create_enumerated_storage(
        allowed_values: Vec<String>,
        default_value: &str,
    ) -> ColumnStorage {
        ColumnStorage::EnumeratedStorage {
            column: Column::new("flavor", "Flavor", default_value),
            data: vec![],
            allowed_values,
        }
    }

    #[test]
    fn test_string_storage() {
        let mut storage = create_string_storage("unknown");

        assert_eq!(storage.add_value("hello"), Ok(None));
        assert_eq!(storage.get_as_string(0), Ok("hello".to_string()));
        assert_eq!(storage.add_value(""), Ok(None));
        assert_eq!(storage.get_as_string(1), Ok("unknown".to_string()));

        let mut storage = create_string_storage("");
        assert_eq!(storage.add_value("hello"), Ok(None));
        assert_eq!(storage.get_as_string(0), Ok("hello".to_string()));
        assert_eq!(storage.add_value(""), Ok(None));
        assert_eq!(storage.get_as_string(1), Ok("".to_string()));
    }

    #[test]
    fn test_boolean_storage() {
        let mut storage = create_boolean_storage("false");

        assert_eq!(storage.add_value("true"), Ok(None));
        assert_eq!(storage.get_as_string(0), Ok("true".to_string()));

        assert_eq!(storage.add_value("tRue"), Ok(None));
        assert_eq!(storage.get_as_string(1), Ok("true".to_string()));

        assert!(storage.add_value("xyz").is_err());
        assert_eq!(storage.get_as_string(2), Ok("".to_string()));

        assert_eq!(storage.add_value(""), Ok(None));
        assert_eq!(storage.get_as_string(3), Ok("false".to_string()));

        let mut storage = create_boolean_storage("");
        assert_eq!(storage.add_value(""), Ok(None));
        assert_eq!(storage.get_as_string(0), Ok("".to_string()));

        assert_eq!(storage.add_value(" TRue "), Ok(None));
        assert_eq!(storage.get_as_string(1), Ok("true".to_string()));
    }

    #[test]
    fn test_f64_storage() {
        let mut storage = create_f64_storage("0");

        assert_eq!(storage.add_value("1.0"), Ok(None));
        assert_eq!(storage.get_as_string(0), Ok("1".to_string()));
        assert_eq!(storage.add_value(""), Ok(None));
        assert_eq!(storage.get_as_string(1), Ok("0".to_string()));
        assert!(storage.add_value("xyz").is_err());
        assert_eq!(storage.get_as_string(2), Ok("".to_string()));
        assert_eq!(storage.add_value("1.3"), Ok(None));
        assert_eq!(storage.get_as_string(3), Ok("1.3".to_string()));

        let mut storage = create_f64_storage("0");
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
    fn test_enumerated_storage() {
        let mut storage = create_enumerated_storage(
            vec![
                "red".to_string(),
                "green".to_string(),
                "blue".to_string(),
                "purple".to_string(),
            ],
            "purple",
        );

        assert_eq!(storage.add_value("red"), Ok(None));
        assert_eq!(storage.add_value("green"), Ok(None));
        assert_eq!(storage.add_value("blue"), Ok(None));
        assert_eq!(storage.add_value("xyz"), Err(CacheError::IllegalState {}));
        assert_eq!(storage.add_value(""), Ok(None));
        assert_eq!(storage.get_as_string(0), Ok("red".to_string()));
        assert_eq!(storage.get_as_string(1), Ok("green".to_string()));
        assert_eq!(storage.get_as_string(2), Ok("blue".to_string()));
        assert_eq!(storage.get_as_string(3), Ok("".to_string()));
        assert_eq!(storage.get_as_string(4), Ok("purple".to_string()));
    }

    #[test]
    fn test_remove_last_value() {
        let mut storage = create_string_storage("unknown");
        storage.add_value("hello").unwrap();
        storage.remove_last_value();
        storage.remove_last_value();
    }
}

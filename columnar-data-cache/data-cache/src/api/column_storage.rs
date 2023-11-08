use crate::api::column::Column;
use crate::api::parsers::{parse_bool, parse_f64, TypeParseError};

pub enum ColumnStorage {
    BooleanStorage {
        column: Column,
        data: Vec<Option<bool>>,
    },
    F64Storage {
        column: Column,
        data: Vec<Option<f64>>,
    },
}

impl ColumnStorage {
    pub fn add_value(&mut self, value: &str) -> Result<Option<f64>, TypeParseError> {
        match self {
            ColumnStorage::BooleanStorage { data, column } => {
                let parsed_value = parse_bool(value, column.default_value.as_str());
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
            ColumnStorage::F64Storage { data, column } => {
                let parsed_value = parse_f64(value, column.default_value.as_str());
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
        }
    }

    pub fn get_length(&self) -> usize {
        match self {
            ColumnStorage::BooleanStorage { data, .. } => data.len(),
            ColumnStorage::F64Storage { data, .. } => data.len(),
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
            column: column,
            data: vec![],
        };

        let result = storage.add_value("true");
        assert_eq!(result, Ok(None));
        assert_eq!(storage.get_as_string(0), "true");
    }
}

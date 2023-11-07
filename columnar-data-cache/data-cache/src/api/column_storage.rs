use crate::api::column_definition::ColumnDefinition;
use crate::api::column_type::{StringColumnType, TypeParseError};

pub struct StringColumnStorage {
    column_definition: ColumnDefinition,
    column_type: StringColumnType,
    data: Vec<String>,
}

impl StringColumnStorage {
    pub fn new(column_definition: ColumnDefinition) -> Self {
        Self {
            column_definition,
            column_type: StringColumnType {},
            data: Vec::new(),
        }
    }

    pub fn get_value(&self, index: usize) -> Option<&String> {
        self.data.get(index)
    }

    pub fn add(&mut self, value: String) -> Result<Option<&String>, TypeParseError> {
        let parse_result = self
            .column_type
            .parse(self.column_definition.get_default_value(), value.as_str());

        match parse_result {
            Ok(the_value) => {
                self.data.push(the_value.unwrap());
                Ok(self.data.last())
            }
            Err(the_error) => Err(the_error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_column_storage() {
        let mut column_storage = StringColumnStorage::new(ColumnDefinition::new(
            "name".parse().unwrap(),
            "The Name".parse().unwrap(),
            "-".parse().unwrap(),
        ));
        assert_eq!(column_storage.get_value(0), None);
        assert_eq!(
            column_storage.add("Joe".parse().unwrap()),
            Ok(Some(&"Joe".to_string()))
        );
        assert_eq!(column_storage.get_value(0), Some(&"Joe".to_string()));
        assert_eq!(
            column_storage.add("".parse().unwrap()),
            Ok(Some(&"-".to_string()))
        );
        assert_eq!(column_storage.get_value(1), Some(&"-".to_string()));
    }
}

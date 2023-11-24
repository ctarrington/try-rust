use crate::api::column::Column;
use crate::api::column_storage::ColumnStorage;
use crate::api::parsers::TypeParseError;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct CacheAccessError {}
pub struct Cache {
    column_stores: Vec<ColumnStorage>,
    guids: Vec<Uuid>,
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            column_stores: vec![],
            guids: vec![],
        }
    }

    fn fill_in_column_store(&self, column_store: &mut ColumnStorage, default_value: &str) {
        if !self.column_stores.is_empty() {
            let row_count = self.column_stores.get(0).unwrap().get_length();

            for _ in 0..row_count {
                column_store.add_value(default_value).unwrap();
            }
        }
    }

    pub fn add_string_column(&mut self, name: &str, display_name: &str, default_value: &str) {
        let mut new_column_store = ColumnStorage::StringStorage {
            column: Column::new(name, display_name, default_value),
            data: vec![],
        };

        self.fill_in_column_store(&mut new_column_store, default_value);
        self.column_stores.push(new_column_store);
    }

    pub fn add_boolean_column(&mut self, name: &str, display_name: &str, default_value: &str) {
        let mut new_column_store = ColumnStorage::BooleanStorage {
            column: Column::new(name, display_name, default_value),
            data: vec![],
        };

        self.fill_in_column_store(&mut new_column_store, default_value);
        self.column_stores.push(new_column_store);
    }

    pub fn add_f64_column(&mut self, name: &str, display_name: &str, default_value: &str) {
        let mut new_column_store = ColumnStorage::F64Storage {
            column: Column::new(name, display_name, default_value),
            data: vec![],
        };

        self.fill_in_column_store(&mut new_column_store, default_value);
        self.column_stores.push(new_column_store);
    }

    pub fn add_time_date_column(
        &mut self,
        name: &str,
        display_name: &str,
        format: &str,
        default_value: &str,
    ) {
        let mut new_column_store = ColumnStorage::TimeDateStorage {
            column: Column::new(name, display_name, ""),
            data: vec![],
            format: format.parse().unwrap(),
        };

        self.fill_in_column_store(&mut new_column_store, default_value);
        self.column_stores.push(new_column_store);
    }

    /// if the default value is not in the allowed values, it will be added
    pub fn add_enumerated_column(
        &mut self,
        name: &str,
        display_name: &str,
        default_value: &str,
        allowed_values: Vec<String>,
    ) {
        let mut full_allowed_values = allowed_values.clone();
        if !allowed_values.contains(&default_value.to_string()) {
            full_allowed_values.push(default_value.to_string());
        }

        let mut new_column_store = ColumnStorage::EnumeratedStorage {
            column: Column::new(name, display_name, default_value),
            data: vec![],
            allowed_values: full_allowed_values,
        };
        self.fill_in_column_store(&mut new_column_store, default_value);
        self.column_stores.push(new_column_store);
    }

    pub fn add_row(&mut self, row: &str) -> Result<Uuid, TypeParseError> {
        let mut error: Option<TypeParseError> = None;
        let values: Vec<&str> = row.split(',').collect();

        if values.len() != self.column_stores.len() {
            return Err(TypeParseError {});
        }

        for (index, value) in values.iter().enumerate() {
            let column_store = self.column_stores.get_mut(index).unwrap();
            let result = column_store.add_value(value);
            if result.is_err() {
                error = Some(result.err().unwrap());
            }
        }

        if let Some(error) = error {
            for column_store in self.column_stores.iter_mut() {
                column_store.remove_last_value();
            }
            Err(error)
        } else {
            let guid = Uuid::new_v4();
            self.guids.push(guid);
            Ok(guid)
        }
    }

    pub fn row_as_csv(&self, index: usize) -> Result<String, CacheAccessError> {
        if self.column_stores.is_empty() {
            return Err(CacheAccessError {});
        }

        if self.column_stores.get(0).unwrap().get_length() <= index {
            return Err(CacheAccessError {});
        }

        let mut row: String = self
            .column_stores
            .iter()
            .map(|column_store| column_store.get_as_string(index).unwrap() + ",")
            .collect();
        row.pop(); // remove the last comma
        Ok(row)
    }
}

// tests are in tests/integration_test_cache.rs since cache is intended for external use

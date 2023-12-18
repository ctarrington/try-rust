pub use crate::api::cache_error::CacheError;
use crate::api::column::Column;
use crate::api::column_metadata::ColumnMetadata;
use crate::api::column_storage::{ColumnStorage, ColumnStorageDataType};
use uuid::Uuid;

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

    pub fn row_count(&self) -> usize {
        if self.column_stores.is_empty() {
            return 0;
        }

        self.column_stores.get(0).unwrap().get_length()
    }

    fn check_for_duplicate_column(&self, name: &str) -> Result<(), CacheError> {
        if self
            .column_stores
            .iter()
            .any(|column_store| column_store.get_column().name == name)
        {
            return Err(CacheError::DuplicateColumn(name.to_string()));
        }

        Ok(())
    }

    pub fn add_string_column(
        &mut self,
        name: &str,
        display_name: &str,
        default_value: &str,
    ) -> Result<(), CacheError> {
        self.check_for_duplicate_column(name)?;
        let mut new_column_store = ColumnStorage::StringStorage {
            column: Column::new(name, display_name, default_value),
            data: vec![],
            column_type: ColumnStorageDataType::String,
        };

        self.fill_in_column_store(&mut new_column_store);
        self.column_stores.push(new_column_store);

        Ok(())
    }

    pub fn add_boolean_column(
        &mut self,
        name: &str,
        display_name: &str,
        default_value: &str,
    ) -> Result<(), CacheError> {
        self.check_for_duplicate_column(name)?;
        let mut new_column_store = ColumnStorage::BooleanStorage {
            column: Column::new(name, display_name, default_value),
            data: vec![],
            column_type: ColumnStorageDataType::Boolean,
        };

        self.fill_in_column_store(&mut new_column_store);
        self.column_stores.push(new_column_store);

        Ok(())
    }

    pub fn add_f64_column(
        &mut self,
        name: &str,
        display_name: &str,
        default_value: &str,
    ) -> Result<(), CacheError> {
        self.check_for_duplicate_column(name)?;
        let mut new_column_store = ColumnStorage::F64Storage {
            column: Column::new(name, display_name, default_value),
            data: vec![],
            column_type: ColumnStorageDataType::F64,
        };

        self.fill_in_column_store(&mut new_column_store);
        self.column_stores.push(new_column_store);

        Ok(())
    }

    pub fn add_time_date_column(
        &mut self,
        name: &str,
        display_name: &str,
        format: &str,
        default_value: &str,
    ) -> Result<(), CacheError> {
        self.check_for_duplicate_column(name)?;
        let mut new_column_store = ColumnStorage::TimeDateStorage {
            column: Column::new(name, display_name, default_value),
            data: vec![],
            format: format.parse().unwrap(),
            column_type: ColumnStorageDataType::TimeDate,
        };

        self.fill_in_column_store(&mut new_column_store);
        self.column_stores.push(new_column_store);

        Ok(())
    }

    /// if the default value is not in the allowed values, it will be added
    pub fn add_enumerated_column(
        &mut self,
        name: &str,
        display_name: &str,
        default_value: &str,
        allowed_values: Vec<String>,
    ) -> Result<(), CacheError> {
        self.check_for_duplicate_column(name)?;
        let mut full_allowed_values = allowed_values.clone();
        if !default_value.is_empty() && !full_allowed_values.contains(&default_value.to_string()) {
            full_allowed_values.push(default_value.to_string());
        }

        let mut new_column_store = ColumnStorage::EnumeratedStorage {
            column: Column::new(name, display_name, default_value),
            data: vec![],
            allowed_values: full_allowed_values,
            column_type: ColumnStorageDataType::Enumerated,
        };

        self.fill_in_column_store(&mut new_column_store);
        self.column_stores.push(new_column_store);

        Ok(())
    }

    pub fn update_row(&mut self, guid: &Uuid, row: &str) -> Result<Uuid, CacheError> {
        let index = self.find_index(guid)?;

        self.add_row(guid, row)?; // returns error if row is invalid
        self.remove_row_by_index(index)?;

        Ok(*guid)
    }
    pub fn create_row(&mut self, row: &str) -> Result<Uuid, CacheError> {
        let guid = Uuid::new_v4();
        self.add_row(&guid, row)
    }

    pub fn csv_for_guid(&self, guid: &Uuid) -> Result<String, CacheError> {
        let index = self.find_index(guid)?;
        self.csv_for_index(index)
    }

    fn fill_in_column_store(&self, column_store: &mut ColumnStorage) {
        let default_value = column_store.get_default_value();
        for _ in 0..self.row_count() {
            column_store.add_value(&default_value).unwrap();
        }
    }

    fn find_index(&self, guid: &Uuid) -> Result<usize, CacheError> {
        match self.guids.iter().position(|g| g == guid) {
            Some(index) => Ok(index),
            None => Err(CacheError::GuidNotFound(*guid)),
        }
    }

    fn remove_row_by_index(&mut self, index: usize) -> Result<(), CacheError> {
        if self.column_stores.is_empty() {
            return Err(CacheError::IllegalState {});
        }

        if self.row_count() <= index {
            return Err(CacheError::IllegalState {});
        }

        for column_store in self.column_stores.iter_mut() {
            column_store.remove_value(index).unwrap();
        }

        self.guids.remove(index);
        Ok(())
    }

    fn add_row(&mut self, guid: &Uuid, row: &str) -> Result<Uuid, CacheError> {
        let mut error: Option<CacheError> = None;
        let values: Vec<&str> = row.split(',').collect();

        if values.len() != self.column_stores.len() {
            return Err(CacheError::IllegalState {});
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
            self.guids.push(*guid);
            Ok(*guid)
        }
    }

    fn csv_for_index(&self, index: usize) -> Result<String, CacheError> {
        if self.column_stores.is_empty() {
            return Err(CacheError::IllegalState {});
        }

        if self.row_count() <= index {
            return Err(CacheError::IllegalState {});
        }

        let row: String = self
            .column_stores
            .iter()
            .map(|column_store| column_store.get_as_string(index).unwrap())
            .collect::<Vec<_>>()
            .join(",");
        Ok(row)
    }

    pub fn get_metadata(&self) -> Vec<ColumnMetadata> {
        self.column_stores.iter().map(ColumnMetadata::new).collect()
    }

    pub fn row_len(&self) -> usize {
        if self.column_stores.is_empty() {
            return 0;
        }

        self.column_stores.get(0).unwrap().get_length()
    }

    pub fn column_len(&self) -> usize {
        self.column_stores.len()
    }
}

// tests are in tests/integration_test_cache.rs since cache is intended for external use

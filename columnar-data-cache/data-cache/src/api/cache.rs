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

    pub fn add_string_column(&mut self, name: &str, display_name: &str, default_value: &str) {
        self.column_stores.push(ColumnStorage::StringStorage {
            column: Column::new(name, display_name, default_value),
            data: vec![],
        });
    }

    pub fn add_boolean_column(&mut self, name: &str, display_name: &str, default_value: &str) {
        self.column_stores.push(ColumnStorage::BooleanStorage {
            column: Column::new(name, display_name, default_value),
            data: vec![],
        });
    }

    pub fn add_f64_column(&mut self, name: &str, display_name: &str, default_value: &str) {
        self.column_stores.push(ColumnStorage::F64Storage {
            column: Column::new(name, display_name, default_value),
            data: vec![],
        });
    }

    pub fn add_time_date_column(&mut self, name: &str, display_name: &str, format: &str) {
        self.column_stores.push(ColumnStorage::TimeDateStorage {
            column: Column::new(name, display_name, ""),
            data: vec![],
            format: format.parse().unwrap(),
        });
    }

    pub fn add_enumerated_column(
        &mut self,
        name: &str,
        display_name: &str,
        default_value: &str,
        allowed_values: Vec<String>,
    ) {
        self.column_stores.push(ColumnStorage::EnumeratedStorage {
            column: Column::new(name, display_name, default_value),
            data: vec![],
            allowed_values,
        });
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_cache() -> Cache {
        let flavors = vec![
            "vanilla".to_string(),
            "chocolate".to_string(),
            "strawberry".to_string(),
        ];
        let mut cache = Cache::new();
        cache.add_string_column("name", "Name", "unknown");
        cache.add_boolean_column("verified", "Verified", "false");
        cache.add_f64_column("age", "Age", "0");
        cache.add_time_date_column("start_time", "Start Time", "%Y-%m-%d %H:%M:%S");
        cache.add_enumerated_column("flavor", "Flavor", "vanilla", flavors);
        cache
    }

    #[test]
    fn test_simple() {
        let mut cache = create_cache();

        assert!(cache.row_as_csv(0).is_err());
        cache
            .add_row("fred,true, 1, 2019-01-01 00:00:00,chocolate")
            .unwrap();
        cache.add_row(",,,,").unwrap();

        assert_eq!(
            cache.row_as_csv(0).unwrap(),
            "fred,true,1,2019-01-01 00:00:00,chocolate"
        );

        assert_eq!(cache.row_as_csv(1).unwrap(), "unknown,false,0,,vanilla");
    }

    #[test]
    fn test_invalid_rows() {
        let mut cache = create_cache();

        assert!(cache.add_row("wilma,false, 2020-01-01 00:00:00,1").is_err());
        assert!(cache.add_row("").is_err());
        assert!(cache.add_row("wilma,false, 2020-01-01 00:00:00,").is_err());
    }

    #[test]
    fn test_empty() {
        let mut cache = create_cache();

        cache.add_row(",,,,").unwrap();
        assert_eq!(cache.row_as_csv(0).unwrap(), "unknown,false,0,,vanilla");
    }

    #[test]
    fn test_valid_after_invalid() {
        let mut cache = create_cache();

        assert!(cache.add_row("wilma,false,1").is_err());
        cache
            .add_row("fred,true, 1, 2019-01-01 00:00:00,strawberry")
            .unwrap();
        assert_eq!(
            cache.row_as_csv(0).unwrap(),
            "fred,true,1,2019-01-01 00:00:00,strawberry"
        );
    }
}

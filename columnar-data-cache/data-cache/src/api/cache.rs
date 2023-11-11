use crate::api::column_storage::ColumnStorage;
use crate::api::parsers::TypeParseError;

#[derive(Debug, PartialEq)]
pub struct CacheAccessError {}
pub struct Cache {
    column_stores: Vec<ColumnStorage>,
}

impl Cache {
    pub fn add_row(&mut self, row: &str) -> Result<(), TypeParseError> {
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
            Ok(())
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
            .map(|column_store| column_store.get_as_string(index) + ",")
            .collect();
        row.pop(); // remove the last comma
        Ok(row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::column::Column;

    #[test]
    fn test_simple() {
        let mut cache = Cache {
            column_stores: vec![
                ColumnStorage::StringStorage {
                    column: Column::new("name", "Name", "unknown"),
                    data: vec![],
                },
                ColumnStorage::BooleanStorage {
                    column: Column::new("verified", "Verified", "false"),
                    data: vec![],
                },
                ColumnStorage::F64Storage {
                    column: Column::new("age", "Age", "0"),
                    data: vec![],
                },
                ColumnStorage::TimeDateStorage {
                    column: Column::new("starttime", "Start Time", ""),
                    data: vec![],
                    format: "%Y-%m-%d %H:%M:%S".parse().unwrap(),
                },
            ],
        };

        assert!(cache.row_as_csv(0).is_err());
        cache.add_row("fred,true, 1, 2019-01-01 00:00:00").unwrap();
        assert_eq!(
            cache.row_as_csv(0).unwrap(),
            "fred,true,1,2019-01-01 00:00:00"
        );
        assert!(cache.add_row("wilma,false, 2020-01-01 00:00:00").is_err());

        assert_eq!(
            cache.row_as_csv(0).unwrap(),
            "fred,true,1,2019-01-01 00:00:00"
        );
    }
}

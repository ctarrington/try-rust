use crate::api::column_storage::ColumnStorage;
use crate::api::parsers::TypeParseError;

#[derive(Debug, PartialEq)]
pub struct CacheAccessError {}
pub struct Cache {
    column_stores: Vec<ColumnStorage>,
}

impl Cache {
    pub fn add_row(&mut self, row: &str) -> Option<TypeParseError> {
        let mut error: Option<TypeParseError> = None;
        let values: Vec<&str> = row.split(",").collect();

        // todo check for mismatch between column count and passed count
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
            Some(error)
        } else {
            None
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
            .into_iter()
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
        let column1 = Column::new("verified", "Verified", "false");
        let column2 = Column::new("age", "Age", "0");

        let mut cache = Cache {
            column_stores: vec![
                ColumnStorage::BooleanStorage {
                    column: column1,
                    data: vec![],
                },
                ColumnStorage::F64Storage {
                    column: column2,
                    data: vec![],
                },
            ],
        };

        assert!(cache.row_as_csv(0).is_err());
        cache.add_row("true, 1");
        assert_eq!(cache.row_as_csv(0).unwrap(), "true,1");
    }
}

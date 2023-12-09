use crate::api::column_storage::{ColumnStorage, ColumnStorageDataType};

pub struct ColumnMetadata {
    pub name: String,
    pub display_name: String,
    pub default_value: String,
    pub data_type: ColumnStorageDataType,
    pub format: String,
}

impl ColumnMetadata {
    pub fn new(column_storage: &ColumnStorage) -> Self {
        let column = column_storage.get_column();
        ColumnMetadata {
            name: column.name.clone(),
            display_name: column.display_name.clone(),
            default_value: column.default_value.clone(),
            format: column_storage.get_format(),
            data_type: column_storage.get_data_type().clone(),
        }
    }
}

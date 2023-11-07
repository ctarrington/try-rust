/// A column definition defines an immutable data container for the name, view name, and default value of a column.

pub struct ColumnDefinition {
    name: String,
    view_name: String,
    default_value: String,
}

impl ColumnDefinition {
    pub fn new(name: String, view_name: String, default_value: String) -> Self {
        Self {
            name,
            view_name,
            default_value,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_view_name(&self) -> &str {
        &self.view_name
    }

    pub fn get_default_value(&self) -> &str {
        &self.default_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_string_column_definition() {
        let column_definition = ColumnDefinition::new(
            "name".parse().unwrap(),
            "The Name".parse().unwrap(),
            "-".parse().unwrap(),
        );
        assert_eq!(column_definition.get_name(), "name");
        assert_eq!(column_definition.get_view_name(), "The Name");
        assert_eq!(column_definition.get_default_value(), "-");
    }
}

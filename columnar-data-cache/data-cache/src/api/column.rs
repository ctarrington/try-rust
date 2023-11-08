pub struct Column {
    pub name: String,
    pub display_name: String,
    pub default_value: String,
}

impl Column {
    pub fn new(name: &str, display_name: &str, default_value: &str) -> Self {
        Column {
            name: name.to_string(),
            display_name: display_name.to_string(),
            default_value: default_value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let column1 = Column::new("verified", "Verified", "false");
        assert_eq!(column1.name, "verified");
        assert_eq!(column1.display_name, "Verified");
        assert_eq!(column1.default_value, "false");
    }
}

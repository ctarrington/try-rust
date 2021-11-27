#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::{Result, Value};

    fn get_raw() -> &'static str {
        r#"
            {"name": "Fred"}
        "#
    }

    #[test]
    fn parse_as_value() -> Result<()> {
        let raw = get_raw();
        let parsed_value: Value = serde_json::from_str(raw)?;
        assert_eq!(parsed_value["name"], "Fred");
        assert_eq!(parsed_value["address"]["street1"], Value::Null);
        Ok(())
    }

    #[test]
    fn parse_as_object() -> Result<()> {
        #[derive(Deserialize, Serialize)]
        struct Contact {
            name: String,
        }

        let raw = get_raw();
        let fred: Contact = serde_json::from_str(raw)?;
        assert_eq!(fred.name, "Fred");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Result, Value};

    #[test]
    fn parse_raw_contact() -> Result<()> {
        let raw = r#"
        {"fred": 
            {"name": "Fred"}
        }
        "#;

        println!("raw: {}", raw);

        let parsed_value: Value = serde_json::from_str(raw)?;
        assert_eq!(parsed_value["fred"]["name"], "Fred");
        assert_eq!(parsed_value["fred"]["address"]["street1"], Value::Null);
        Ok(())
    }
}

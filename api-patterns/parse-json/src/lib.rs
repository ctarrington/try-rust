#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::{Result, Value};

    #[derive(Deserialize, Serialize)]
    struct Address {
        street1: String,
        street2: String,
        city: String,
        state: String,
        zip: String,
    }

    #[derive(Deserialize, Serialize)]
    struct Contact {
        name: String,
        address: Address,
    }

    fn get_raw() -> &'static str {
        r#"
            {"name": "Fred",
            "address": {
                "street1": "123 Main Street",
                "street2": "",
                "city": "Smalltown",
                "state": "MN",
                "zip": "12345"
                },
            "somemetadata": "this is ignored"
            }
        "#
    }

    #[test]
    fn parse_as_value() -> Result<()> {
        let raw = get_raw();
        let parsed_value: Value = serde_json::from_str(raw)?;
        assert_eq!(parsed_value["name"], "Fred");
        assert_eq!(parsed_value["address"]["street1"], "123 Main Street");
        Ok(())
    }

    #[test]
    fn parse_as_object() -> Result<()> {
        let raw = get_raw();
        let fred: Contact = serde_json::from_str(raw)?;
        assert_eq!(fred.name, "Fred");
        assert_eq!(fred.address.city, "Smalltown");
        Ok(())
    }
}

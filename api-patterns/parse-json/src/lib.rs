use rand::distributions::uniform::Uniform;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Address {
    street1: String,
    street2: String,
    city: String,
    state: String,
    zip: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Contact {
    id: String,
    name: String,
    address: Address,
}

// todo: wrap up in an iterator
fn create_random_contact(id: &str) -> Contact {
    let names: Vec<String> = ["Ted", "Fred", "Barney", "Betty", "Wilma"]
        .iter()
        .map(|&s| s.into())
        .collect();

    let mut rng = rand::thread_rng();
    let uniform_indexes = Uniform::from(0..names.len());
    let index = uniform_indexes.sample(&mut rng);
    let name = names.get(index).unwrap().to_string();

    let address = Address {
        street1: "123 Main Street".to_string(),
        street2: "".to_string(),
        city: "nowhere".to_string(),
        state: "md".to_string(),
        zip: "21228".to_string(),
    };

    Contact {
        id: id.to_string(),
        name,
        address,
    }
}

#[cfg(test)]
mod tests {
    use crate::{create_random_contact, Contact};
    use serde_json::{Result, Value};

    fn get_raw() -> &'static str {
        r#"
            {"id": "123",
            "name": "Fred",
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

    #[test]
    fn create_contact_json() {
        let contact = create_random_contact("123");
        println!("contact {:?}", contact);
        assert_eq!(contact.id, "123");
    }
}

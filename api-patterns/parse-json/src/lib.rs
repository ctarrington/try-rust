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

struct RandomStringIterator {
    values: Vec<String>,
    rng: ThreadRng,
}

impl RandomStringIterator {
    fn new(values: Vec<String>) -> Self {
        Self {
            values,
            rng: rand::thread_rng(),
        }
    }
}

impl Iterator for RandomStringIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.values
            .choose(&mut self.rng)
            .map(|value| value.to_string())
    }
}

struct RandomContactIterator {
    stop_id: u32,
    current_id: u32,
    name_iterator: RandomStringIterator,
}

impl RandomContactIterator {
    fn new(start_id: u32, stop_id: u32) -> Self {
        let names: Vec<String> = ["Ted", "Fred", "Barney", "Betty", "Wilma"]
            .iter()
            .map(|&s| s.into())
            .collect();

        Self {
            stop_id,
            current_id: start_id,
            name_iterator: RandomStringIterator::new(names),
        }
    }
}

impl Iterator for RandomContactIterator {
    type Item = Contact;

    fn next(&mut self) -> Option<Self::Item> {
        let name = self.name_iterator.next().unwrap();
        let address = Address {
            street1: "123 Main Street".to_string(),
            street2: "".to_string(),
            city: "nowhere".to_string(),
            state: "md".to_string(),
            zip: "21228".to_string(),
        };

        let contact = Some(Contact {
            id: self.current_id.to_string(),
            name,
            address,
        });

        if self.current_id > self.stop_id {
            return None;
        }
        self.current_id += 1;

        contact
    }
}

#[cfg(test)]
mod tests {
    use crate::{Contact, RandomContactIterator};
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
        let mut contact_iterator = RandomContactIterator::new(0, 2);
        let contact = contact_iterator.next();
        assert!(contact.is_some());
        assert_eq!(contact.unwrap().id, "0");

        let contact = contact_iterator.next();
        assert!(contact.is_some());
        assert_eq!(contact.unwrap().id, "1");

        let contact = contact_iterator.next();
        assert!(contact.is_some());
        println!("contact {:?}", contact);
        assert_eq!(contact.unwrap().id, "2");

        let contact = contact_iterator.next();
        assert!(contact.is_none());
    }
}

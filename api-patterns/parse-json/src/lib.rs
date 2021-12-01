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
    street_number_iterator: RandomStringIterator,
    street_name_iterator: RandomStringIterator,
}

fn list_to_vector(values: &[&str]) -> Vec<String> {
    values.iter().map(|&s| s.into()).collect()
}

impl RandomContactIterator {
    fn new(start_id: u32, stop_id: u32) -> Self {
        let names: Vec<String> = list_to_vector(&["Ted", "Fred", "Barney", "Betty", "Wilma"]);
        let street_numbers: Vec<String> = list_to_vector(&["123", "1", "431", "3131", "111"]);
        let street_names: Vec<String> =
            list_to_vector(&["Main Street", "Water Street", "Winding Blvd", "Saddle Ct"]);

        Self {
            stop_id,
            current_id: start_id,
            name_iterator: RandomStringIterator::new(names),
            street_number_iterator: RandomStringIterator::new(street_numbers),
            street_name_iterator: RandomStringIterator::new(street_names),
        }
    }
}

impl Iterator for RandomContactIterator {
    type Item = Contact;

    fn next(&mut self) -> Option<Self::Item> {
        let name = self.name_iterator.next().unwrap();
        let street_number = self.street_number_iterator.next().unwrap();
        let street_name = self.street_name_iterator.next().unwrap();

        let address = Address {
            street1: street_number + " " + &street_name,
            street2: "".to_string(),
            city: "Nowhere".to_string(),
            state: "MD".to_string(),
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

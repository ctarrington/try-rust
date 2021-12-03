use parse_json::{Contact, RandomContactIterator};

fn main() -> Result<(), std::io::Error> {
    let mut contact_iterator = RandomContactIterator::new(0, 3);

    while let Some(contact) = contact_iterator.next() {
        let serialized_contact = serde_json::to_string(&contact)?;
        println!("contact {:?}", serialized_contact);
        let deserialized_contact: Contact = serde_json::from_str(&serialized_contact)?;
        println!("deserialized_contact {:?}", deserialized_contact);
    }

    Ok(())
}

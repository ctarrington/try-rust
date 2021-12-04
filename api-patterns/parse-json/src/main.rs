use parse_json::{
    create_and_write_contacts, read_contact, write_contact, Contact, RandomContactIterator,
};

fn main() -> Result<(), std::io::Error> {
    let mut contact_iterator = RandomContactIterator::new(0, 3);

    while let Some(contact) = contact_iterator.next() {
        let serialized_contact = serde_json::to_string(&contact)?;
        println!("contact {:?}", serialized_contact);
        let deserialized_contact: Contact = serde_json::from_str(&serialized_contact)?;
        println!("deserialized_contact {:?}", deserialized_contact);
        write_contact(contact)?;
    }

    let read_contact = read_contact(0u32);
    println!("read_contact {:?}", read_contact);

    create_and_write_contacts(55u32, 60u32)?;

    Ok(())
}

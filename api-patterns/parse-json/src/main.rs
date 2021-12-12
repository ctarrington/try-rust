use parse_json::{
    create_and_write_contacts_concurrent, ensure_clean_path, read_contacts_concurrent, Contact,
};

fn main() -> Result<(), std::io::Error> {
    ensure_clean_path()?;

    let contact_count = 1000;
    let thread_count = 4;

    create_and_write_contacts_concurrent(0, contact_count, thread_count)?;
    let contacts: Vec<Contact> = read_contacts_concurrent(0, contact_count, thread_count)?;

    println!("wrote and read {} contacts", contacts.len());
    println!("contact.get(0) = {:?}", contacts.get(0).unwrap());
    println!("contact.get(100) = {:?}", contacts.get(100).unwrap());

    Ok(())
}

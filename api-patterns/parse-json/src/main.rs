use parse_json::{
    create_and_write_contacts_concurrent, ensure_clean_path, read_contacts_concurrent, Contact,
};

fn main() -> Result<(), std::io::Error> {
    ensure_clean_path()?;

    create_and_write_contacts_concurrent(0, 100, 4)?;
    let contacts: Vec<Contact> = read_contacts_concurrent(0, 100, 4)?;

    println!("wrote and read {} contacts", contacts.len());
    println!("contact.get(100) = {:?}", contacts.get(100));

    Ok(())
}

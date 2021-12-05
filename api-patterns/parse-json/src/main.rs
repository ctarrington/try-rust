use parse_json::create_and_write_contacts_concurrent;

fn main() -> Result<(), std::io::Error> {
    create_and_write_contacts_concurrent(2u32, 100u32, 4u32)?;

    Ok(())
}

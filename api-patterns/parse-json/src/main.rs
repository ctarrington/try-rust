use parse_json::{create_and_write_contacts_concurrent, ensure_clean_path};

fn main() -> Result<(), std::io::Error> {
    ensure_clean_path()?;
    create_and_write_contacts_concurrent(0u32, 100u32, 4u32)?;

    Ok(())
}

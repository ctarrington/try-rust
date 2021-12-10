use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::fs;
use std::path::Path;
use std::thread;
use std::thread::JoinHandle;

const OUTPUT_PATH: &str = "./output/contacts/";

#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    street1: String,
    street2: String,
    city: String,
    state: String,
    zip: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    id: u32,
    name: String,
    address: Address,
}

/// start and end of the inclusive execution block
struct ExecutionBlock {
    start_index: u32,
    stop_index: u32,
}

/// Calculate a set of inclusive ranges that covers the specified inclusive ranges.
/// The last range may be slightly smaller than the others and the number of blocks can be less
/// than specified if stop - start is too small
fn calculate_execution_blocks(start: u32, stop: u32, number_of_blocks: u32) -> Vec<ExecutionBlock> {
    let mut execution_blocks = Vec::new();

    let raw_stride = (stop as f32 - start as f32) / number_of_blocks as f32;
    let stride = raw_stride.ceil() as u32 + 1;

    let mut start_index = start;
    for _ in 0..number_of_blocks {
        let stop_index = start_index + stride - 1;
        let stop_index = cmp::min(stop_index, stop);
        execution_blocks.push(ExecutionBlock {
            start_index,
            stop_index,
        });

        start_index += stride;

        if start_index > stop {
            break;
        }
    }

    execution_blocks
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

pub struct RandomContactIterator {
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
    pub fn new(start_id: u32, stop_id: u32) -> Self {
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

pub fn ensure_clean_path() -> std::io::Result<()> {
    if Path::new(OUTPUT_PATH).exists() {
        println!("deleting output directory {}", OUTPUT_PATH);
        fs::remove_dir_all(OUTPUT_PATH)?;
        println!("after delete of output directory {}", OUTPUT_PATH);
    }

    println!("creating output directory {}", OUTPUT_PATH);
    fs::create_dir_all(OUTPUT_PATH)?;

    Ok(())
}
fn file_path(id: u32) -> String {
    format!("{}contact{}.out", OUTPUT_PATH, id)
}

pub fn write_contact(contact: Contact) -> Result<(), std::io::Error> {
    let serialized_contact = serde_json::to_string(&contact)?;
    fs::write(file_path(contact.id), serialized_contact)?;
    Ok(())
}

pub fn read_contact(id: u32) -> Result<Contact, std::io::Error> {
    let raw_contact =
        fs::read_to_string(file_path(id)).expect(&format!("Unable to read {}", file_path(id)));
    let contact: Contact = serde_json::from_str(&raw_contact)?;
    Ok(contact)
}
/// create a contact for each id in the range, inclusive. Returns Ok or the first encountered error.
pub fn create_and_write_contacts(start_id: u32, stop_id: u32) -> Result<(), std::io::Error> {
    let mut contact_iterator = RandomContactIterator::new(start_id, stop_id);

    while let Some(contact) = contact_iterator.next() {
        write_contact(contact)?;
    }
    Ok(())
}

pub fn create_and_write_contacts_concurrent(
    start_id: u32,
    stop_id: u32,
    thread_count: u32,
) -> Result<(), std::io::Error> {
    let blocks = calculate_execution_blocks(start_id, stop_id, thread_count);

    let mut handles: Vec<JoinHandle<Result<(), std::io::Error>>> = Vec::new();
    for ExecutionBlock {
        start_index,
        stop_index,
    } in blocks
    {
        let handle = thread::spawn(move || {
            create_and_write_contacts(start_index, stop_index)?;
            println!(
                "create_and_write_contacts: {} to {}",
                start_index, stop_index
            );
            Ok(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()?;
    }

    Ok(())
}

pub fn read_contacts(start_id: u32, stop_id: u32) -> Result<Vec<Contact>, std::io::Error> {
    let mut index = start_id;
    let mut contacts = Vec::new();

    while let Ok(contact) = read_contact(index) {
        println!("contact {:?}", contact);
        contacts.push(contact);
        index += 1;
        if index > stop_id {
            break;
        }
    }

    Ok(contacts)
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
            id: self.current_id,
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
    use crate::{
        calculate_execution_blocks, create_and_write_contacts,
        create_and_write_contacts_concurrent, ensure_clean_path, file_path, read_contact,
        read_contacts, write_contact, Contact, ExecutionBlock, RandomContactIterator,
    };
    use serde_json::Value;
    use serial_test::serial;
    use std::fs::OpenOptions;
    use std::{fs, time};

    fn get_raw() -> &'static str {
        r#"
            {"id": 123,
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

    fn set_readonly_for_contact(id: u32, value: bool) -> Result<(), std::io::Error> {
        let path = file_path(id);
        let _result = OpenOptions::new().create(true).write(true).open(&path);
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_readonly(value);
        fs::set_permissions(&path, perms)?;

        Ok(())
    }

    #[test]
    #[serial]
    fn parse_as_value() -> serde_json::Result<()> {
        let raw = get_raw();
        let parsed_value: Value = serde_json::from_str(raw)?;
        assert_eq!(parsed_value["name"], "Fred");
        assert_eq!(parsed_value["address"]["street1"], "123 Main Street");
        Ok(())
    }

    #[test]
    #[serial]
    fn parse_as_object() -> serde_json::Result<()> {
        let raw = get_raw();
        let fred: Contact = serde_json::from_str(raw)?;
        assert_eq!(fred.name, "Fred");
        assert_eq!(fred.address.city, "Smalltown");
        Ok(())
    }

    #[test]
    #[serial]
    fn create_contact_json() {
        let mut contact_iterator = RandomContactIterator::new(0, 2);
        let contact = contact_iterator.next();
        assert!(contact.is_some());
        assert_eq!(contact.unwrap().id, 0);

        let contact = contact_iterator.next();
        assert!(contact.is_some());
        assert_eq!(contact.unwrap().id, 1);

        let contact = contact_iterator.next();
        assert!(contact.is_some());
        println!("contact {:?}", contact);
        assert_eq!(contact.unwrap().id, 2);

        let contact = contact_iterator.next();
        assert!(contact.is_none());
    }

    #[test]
    #[serial]
    fn read_and_write_contact() -> Result<(), std::io::Error> {
        ensure_clean_path()?;
        let raw = get_raw();
        let fred: Contact = serde_json::from_str(raw)?;

        write_contact(fred)?;
        let recovered_fred = read_contact(123)?;
        assert_eq!(recovered_fred.name, "Fred");
        assert_eq!(recovered_fred.address.city, "Smalltown");

        Ok(())
    }

    #[test]
    #[serial]
    fn bulk_write_bulk_read() -> Result<(), std::io::Error> {
        ensure_clean_path()?;
        create_and_write_contacts(100, 102)?;
        let contacts = read_contacts(100, 102)?;

        assert_eq!(contacts.len(), 3);
        assert_eq!(contacts.get(0).unwrap().id, 100);

        Ok(())
    }

    #[test]
    #[serial]
    fn concurrent_creation() -> Result<(), std::io::Error> {
        ensure_clean_path()?;
        create_and_write_contacts_concurrent(0, 15, 4)?;

        Ok(())
    }

    #[test]
    #[serial]
    fn failed_write() -> Result<(), std::io::Error> {
        ensure_clean_path()?;
        set_readonly_for_contact(1002, true)?;
        assert!(create_and_write_contacts(1001, 1002).is_err());

        set_readonly_for_contact(1002, false)?;
        Ok(())
    }

    #[test]
    #[serial]
    fn failed_write_concurrent() -> Result<(), std::io::Error> {
        ensure_clean_path()?;
        set_readonly_for_contact(1002, true)?;
        assert!(create_and_write_contacts_concurrent(1001, 1010, 2).is_err());

        set_readonly_for_contact(1002, false)?;
        Ok(())
    }

    #[test]
    #[serial]
    fn concurrent_advantage() -> Result<(), std::io::Error> {
        let contact_count = 10000;

        ensure_clean_path()?;
        let begin = time::Instant::now();
        create_and_write_contacts(0, contact_count)?;
        let elapsed = time::Instant::now() - begin;

        std::thread::sleep(time::Duration::from_millis(10));

        ensure_clean_path()?;
        let thread_count = 3;
        let begin = time::Instant::now();
        create_and_write_contacts_concurrent(0, contact_count, thread_count)?;
        let elapsed_concurrent = time::Instant::now() - begin;

        let ratio = elapsed.as_nanos() as f32 / elapsed_concurrent.as_nanos() as f32;

        println!(
            "elapsed: {:?}, elapsed_concurrent: {:?}, ratio: {:?}",
            elapsed, elapsed_concurrent, ratio
        );
        assert!(ratio > 2.5);
        Ok(())
    }

    #[test]
    #[serial]
    fn block_creation() {
        let blocks = calculate_execution_blocks(0, 100, 4);
        assert_eq!(4, blocks.len());
        assert!(matches!(
            blocks.get(0),
            Some(ExecutionBlock {
                start_index: 0,
                stop_index: 25
            })
        ));

        assert!(matches!(
            blocks.get(3),
            Some(ExecutionBlock {
                start_index: 78,
                stop_index: 100
            })
        ));

        let blocks = calculate_execution_blocks(5, 10, 2);
        assert_eq!(2, blocks.len());
        assert!(matches!(
            blocks.get(0),
            Some(ExecutionBlock {
                start_index: 5,
                stop_index: 8
            })
        ));

        assert!(matches!(
            blocks.get(1),
            Some(ExecutionBlock {
                start_index: 9,
                stop_index: 10
            })
        ));

        let blocks = calculate_execution_blocks(5, 10, 7);
        assert_eq!(3, blocks.len());
        assert!(matches!(
            blocks.get(0),
            Some(ExecutionBlock {
                start_index: 5,
                stop_index: 6
            })
        ));

        assert!(matches!(
            blocks.get(2),
            Some(ExecutionBlock {
                start_index: 9,
                stop_index: 10
            })
        ));

        let blocks = calculate_execution_blocks(0, 0, 2);
        assert_eq!(1, blocks.len());
        assert!(matches!(
            blocks.get(0),
            Some(ExecutionBlock {
                start_index: 0,
                stop_index: 0
            })
        ));
    }
}

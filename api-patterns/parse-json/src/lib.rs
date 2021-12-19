use rand::prelude::*;
use rand::Rng;
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
    big_number: u32,
    proof_of_work: u32,
    name: String,
    address: Address,
}

/// start and end of the inclusive execution block
struct ExecutionBlock {
    start_index: u32,
    stop_index: u32,
}

/// simplistic proof of work scheme to introduce a little variability how long a process takes
/// picks a random number until it is at or below a target
fn calculate_proof_of_work(target: u32, range_max: u32) -> u32 {
    let mut rng = rand::thread_rng();

    loop {
        let value: u32 = rng.gen_range(0..range_max);
        if value <= target {
            break value;
        }
    }
}

/// Calculate a set of inclusive ranges that covers the specified inclusive range.
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

pub fn create_contacts(start_id: u32, stop_id: u32) -> Vec<Contact> {
    let mut contact_iterator = RandomContactIterator::new(start_id, stop_id);

    let mut contacts = Vec::new();
    while let Some(contact) = contact_iterator.next() {
        contacts.push(contact);
    }

    contacts
}

pub fn create_contacts_concurrent(start_id: u32, stop_id: u32, thread_count: u32) -> Vec<Contact> {
    let blocks = calculate_execution_blocks(start_id, stop_id, thread_count);

    let mut handles = Vec::new();
    for ExecutionBlock {
        start_index,
        stop_index,
    } in blocks
    {
        let handle = thread::spawn(move || {
            println!("create_contacts: {} to {}", start_index, stop_index);
            create_contacts(start_index, stop_index)
        });
        handles.push(handle);
    }

    let mut contacts = Vec::new();
    for handle in handles {
        contacts.append(
            &mut handle
                .join()
                .expect("error getting contact from create handle"),
        );
    }

    contacts
}

/// Use the specified number of threads to create and write contacts to disk.
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
        handle.join().expect("error joining handles for writes")?;
    }

    Ok(())
}

pub fn read_contacts(start_id: u32, stop_id: u32) -> Result<Vec<Contact>, std::io::Error> {
    let mut index = start_id;
    let mut contacts = Vec::new();

    while let Ok(contact) = read_contact(index) {
        contacts.push(contact);
        index += 1;
        if index > stop_id {
            break;
        }
    }

    Ok(contacts)
}

pub fn read_contacts_concurrent(
    start_id: u32,
    stop_id: u32,
    thread_count: u32,
) -> Result<Vec<Contact>, std::io::Error> {
    let blocks = calculate_execution_blocks(start_id, stop_id, thread_count);

    let mut handles: Vec<JoinHandle<Result<Vec<Contact>, std::io::Error>>> = Vec::new();
    for ExecutionBlock {
        start_index,
        stop_index,
    } in blocks
    {
        let handle = thread::spawn(move || {
            let contacts = read_contacts(start_index, stop_index)?;
            println!("read_contacts {} to {}", start_index, stop_index);
            Ok(contacts)
        });
        handles.push(handle);
    }

    let mut consolidated_contacts = Vec::new();
    for handle in handles {
        let mut contacts = handle.join().expect("error joining read contact handles")?;
        consolidated_contacts.append(&mut contacts);
    }

    Ok(consolidated_contacts)
}

pub fn find_minimum_contact<'a>(
    contacts: &'a Vec<Contact>,
    start_index: u32,
    stop_index: u32,
) -> &'a Contact {
    // I mostly think it makes sense to have assertions like this only enabled in debug mode
    // Should look at the contracts crate
    debug_assert!(
        start_index < stop_index
            && start_index < contacts.len() as u32
            && stop_index < contacts.len() as u32,
        "bad parameters: start_index: {} stop_index: {} contacts length: {}",
        start_index,
        stop_index,
        contacts.len()
    );

    &contacts[start_index as usize..=stop_index as usize]
        .iter()
        .reduce(|min_contact, contact| {
            if min_contact.big_number <= contact.big_number {
                min_contact
            } else {
                contact
            }
        })
        .unwrap()
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

        let proof_of_work = calculate_proof_of_work(3, 10_000);

        let mut rng = rand::thread_rng();

        let big_number: u32 = rng.gen_range(0..1_000_000);
        let contact = Some(Contact {
            id: self.current_id,
            big_number,
            name,
            address,
            proof_of_work,
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
        create_and_write_contacts_concurrent, create_contacts, create_contacts_concurrent,
        ensure_clean_path, file_path, find_minimum_contact, read_contact, read_contacts,
        read_contacts_concurrent, write_contact, Contact, ExecutionBlock, RandomContactIterator,
    };
    use serde_json::Value;
    use serial_test::serial;
    use std::fs::OpenOptions;
    use std::{fs, time};

    fn get_raw() -> &'static str {
        r#"
            {"id": 123,
            "name": "Fred",
            "proof_of_work": 11,
            "big_number": 1000,
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
    fn find_minimum() {
        let contacts = create_contacts(0, 100);
        let min_contact = find_minimum_contact(&contacts, 0, 100);
        let min_value = contacts
            .iter()
            .map(|contact| contact.big_number)
            .min()
            .unwrap();
        assert_eq!(min_value, min_contact.big_number);
    }

    #[test]
    fn parse_as_value() -> serde_json::Result<()> {
        let raw = get_raw();
        let parsed_value: Value = serde_json::from_str(raw)?;
        assert_eq!(parsed_value["name"], "Fred");
        assert_eq!(parsed_value["address"]["street1"], "123 Main Street");
        Ok(())
    }

    #[test]
    fn parse_as_object() -> serde_json::Result<()> {
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
    fn create_contacts_no_io() {
        let contacts = create_contacts(0, 10);
        assert_eq!(contacts.len(), 11);
    }

    #[test]
    fn create_contacts_concurrent_no_io() {
        let contacts = create_contacts_concurrent(0, 10, 4);
        assert_eq!(contacts.len(), 11);
        assert_eq!(contacts.get(0).unwrap().id, 0);
        assert_eq!(contacts.get(10).unwrap().id, 10);
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
    fn concurrent_create_and_write() -> Result<(), std::io::Error> {
        ensure_clean_path()?;
        create_and_write_contacts_concurrent(0, 15, 4)?;
        assert!(matches!(read_contact(0), Ok(Contact { id: 0, .. })));
        assert!(matches!(read_contact(15), Ok(Contact { id: 15, .. })));
        assert!(read_contact(1).unwrap().proof_of_work <= 3);

        let contacts = read_contacts_concurrent(0, 15, 4)?;
        assert!(matches!(contacts.get(0), Some(Contact { id: 0, .. })));
        assert!(matches!(contacts.get(15), Some(Contact { id: 15, .. })));

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
        let contact_count = 1000;

        ensure_clean_path()?;
        let begin = time::Instant::now();
        create_and_write_contacts(0, contact_count)?;
        let elapsed = time::Instant::now() - begin;

        ensure_clean_path()?;
        let thread_count = 4;
        let begin = time::Instant::now();
        create_and_write_contacts_concurrent(0, contact_count, thread_count)?;
        let elapsed_concurrent = time::Instant::now() - begin;

        let ratio = elapsed.as_nanos() as f32 / elapsed_concurrent.as_nanos() as f32;
        let desired_ratio = 1.5;

        println!(
            "elapsed: {:?}, elapsed_concurrent: {:?}, ratio: {:?}, desired ratio: {:?}",
            elapsed, elapsed_concurrent, ratio, desired_ratio
        );
        assert!(ratio > desired_ratio);
        Ok(())
    }

    #[test]
    #[serial]
    fn concurrent_advantage_no_io() {
        let contact_count = 1000;

        let begin = time::Instant::now();
        create_contacts(0, contact_count);
        let elapsed = time::Instant::now() - begin;

        let thread_count = 4;
        let begin = time::Instant::now();
        create_contacts_concurrent(0, contact_count, thread_count);
        let elapsed_concurrent = time::Instant::now() - begin;

        let ratio = elapsed.as_nanos() as f32 / elapsed_concurrent.as_nanos() as f32;
        let desired_ratio = 1.5;

        println!(
            "elapsed: {:?}, elapsed_concurrent for no io: {:?}, ratio: {:?}, desired ratio: {:?}",
            elapsed, elapsed_concurrent, ratio, desired_ratio
        );
        assert!(ratio > desired_ratio);
    }

    #[test]
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

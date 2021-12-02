use parse_json::RandomContactIterator;

fn main() {
    println!("hello from parse-json");
    let mut contact_iterator = RandomContactIterator::new(0, 3);

    while let Some(contact) = contact_iterator.next() {
        println!("contact {:?}", serde_json::to_string(&contact));
    }
}

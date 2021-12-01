use parse_json::RandomContactIterator;

fn main() {
    println!("hello from parse-json");
    let mut contact_iterator = RandomContactIterator::new(0, 2);
    let contact = contact_iterator.next();
    println!("contact {:?}", contact);
}

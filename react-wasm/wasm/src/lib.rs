use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Person {
    first_name: String,
    last_name: String,
}

#[wasm_bindgen]
impl Person {
    pub fn new(first_name: String, last_name: String) -> Self {
        Person {
            first_name,
            last_name,
        }
    }
}

#[wasm_bindgen]
pub fn add_two_ints(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn greet_person(person: Person) -> String {
    format!("Hi, {} {}", person.first_name, person.last_name)
}

#[wasm_bindgen]
pub fn greet(name: String) -> String {
    format!("Hi, {}", name)
}

#[wasm_bindgen]
pub fn format_name(person: Person) -> String {
    format!("{} {}", person.first_name, person.last_name)
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Person {
    first_name: String,
    last_name: String,
    coins: u32,
}

#[wasm_bindgen]
impl Person {
    pub fn new(first_name: String, last_name: String) -> Self {
        Person {
            first_name,
            last_name,
            coins: 0,
        }
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn coins(&self) -> u32 {
        self.coins
    }

    pub fn tick(&mut self) {
        self.coins = self.coins + 1;
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
pub fn get_joe() -> Person {
    Person::new("Joe".to_string(), "Josephson".to_string())
}

#[wasm_bindgen]
pub fn format_name(person: Person) -> String {
    format!("{} {}", person.first_name, person.last_name)
}

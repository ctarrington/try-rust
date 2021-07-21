#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub nick_name: String,
    pub age: u8,
}

impl Person {
    pub fn increase_age(&mut self) {
        self.age += 1;
    }
}

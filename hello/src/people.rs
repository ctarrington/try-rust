use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub nick_name: String,
    pub age: u8,
    pub friend: Option<Box<Person>>,
}

impl Person {
    pub fn increase_age(&mut self) {
        self.age += 1;
    }
}

impl fmt::Display for Person {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{name} is {age} years old. ",
            name = self.name,
            age = self.age
        )
        .unwrap();
        match &self.friend {
            Some(friend) => write!(
                formatter,
                "They have a friend, {} who calls them {}",
                friend.name, self.nick_name
            ),
            _ => write!(formatter, "They are lonely"),
        }
    }
}

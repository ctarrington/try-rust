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

#[test]
fn test_person() {
    let joe = Person {
        name: String::from("Joe"),
        nick_name: String::from("Joey"),
        age: 55,
        friend: None,
    };

    assert_eq!("Joe is 55 years old. They are lonely", format!("{}", joe));
    assert_eq!("Joe", joe.name);
    assert_eq!(55, joe.age);

    // typed destructuring ftw. ignore fields with the dot dot
    // partial move
    let Person {
        name: mut joe_name, ..
    } = joe;

    // assert_eq!("Joe", joe.name); // can not access joe.name anymore since it moved
    // joe.nick_name and joe.age are still fine
    assert_eq!("Joey", joe.nick_name);
    assert_eq!(55, joe.age);

    assert_eq!("Joe", joe_name);
    joe_name = joe_name.to_uppercase();
    assert_eq!("JOE", joe_name);

    let jane = Person {
        name: String::from("Jane"),
        nick_name: String::from("Janey"),
        age: 54,
        friend: Some(Box::new(Person {
            name: String::from("Fred"),
            nick_name: String::from("Freddy"),
            age: 55,
            friend: None,
        })),
    };

    let jane_name = jane.name;
    assert_eq!("Jane", jane_name);
    // assert_eq!("Jane", jane.name); // can not access jane.name since it moved
    assert_eq!(54, jane.age);
    assert_eq!("Janey", jane.nick_name);
    assert_eq!("Freddy", jane.friend.unwrap().nick_name);
}

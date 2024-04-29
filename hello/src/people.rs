use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Person<'a> {
    pub name: String,
    pub nick_name: String,
    pub age: u8,
    pub friend: Option<&'a Person<'a>>,
}

impl Person<'_> {
    pub fn increase_age(&mut self) {
        self.age += 1;
    }
}

impl fmt::Display for Person<'_> {
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

    // typed destructuring ftw. ignore fields with the dot then dot
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

    let fred = Person {
        name: String::from("Fred"),
        nick_name: String::from("Freddy"),
        age: 55,
        friend: None,
    };

    let jane = Person {
        name: String::from("Jane"),
        nick_name: String::from("Janey"),
        age: 54,
        friend: Some(&fred),
    };

    let jane_name_ref = &jane.name;
    assert_eq!("Jane", *jane_name_ref);

    let jane_name = jane.name;
    assert_eq!("Jane", jane_name);
    // assert_eq!("Jane", jane.name); // can not access jane.name since it moved
    assert_eq!(54, jane.age);
    assert_eq!("Janey", jane.nick_name);
    assert_eq!("Freddy", jane.friend.unwrap().nick_name);
}

#[test]
fn test_mutual_friend() {
    let mut joe = Person {
        name: String::from("Joe"),
        nick_name: String::from("Joey"),
        age: 55,
        friend: None,
    };

    assert_eq!("Joe is 55 years old. They are lonely", format!("{}", joe));
    joe.increase_age();
    assert_eq!("Joe is 56 years old. They are lonely", format!("{}", joe));

    let mut jane = Person {
        name: String::from("Jane"),
        nick_name: String::from("Janey"),
        age: 54,
        friend: None,
    };

    assert_eq!("Jane is 54 years old. They are lonely", format!("{}", jane));

    jane.friend = Some(&joe);

    let betty = Person {
        name: String::from("Betty"),
        nick_name: String::from("Bets"),
        age: 53,
        friend: Some(&joe),
    };

    assert_eq!("Joe is 56 years old. They are lonely", format!("{}", joe));
    assert_eq!(
        "Jane is 54 years old. They have a friend, Joe who calls them Janey",
        format!("{}", jane)
    );
    assert_eq!(
        "Betty is 53 years old. They have a friend, Joe who calls them Bets",
        format!("{}", betty)
    );
}

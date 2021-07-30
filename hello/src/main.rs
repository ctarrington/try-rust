use std::mem;

mod people;
use self::people::Person;
// use crate::people::Person; // works as well

mod list;
use self::list::List;

enum Move {
    Up(i8),
    Down(i8),
}

impl Move {
    fn extract_motion(&self) -> i8 {
        match *self {
            Move::Up(value) => value,
            Move::Down(value) => -value,
        }
    }
}

#[test]
fn test_moves() {
    let up2 = Move::Up(2);
    let down3 = Move::Down(3);

    assert_eq!(2, up2.extract_motion());
    assert_eq!(-3, down3.extract_motion());
}

fn main() {
    println!("Hello, world!");
    println!("gcd = {}", gcd(12, 14));

    let mut fred = Person {
        name: String::from("Fred"),
        nick_name: String::from("Freddy"),
        age: 15,
        friend: None,
    };

    println!("fred = {:?}", fred);
    fred.increase_age();
    println!("fred = {:?}", fred);
    // fred.age += 1; // nope private

    println!("move = {}", Move::Up(2).extract_motion());
    println!("move = {}", Move::Down(2).extract_motion());
    let list = List::<u32>::Node {
        value: 12,
        next: Box::new(List::<u32>::Node {
            value: 13,
            next: Box::new(List::<u32>::Empty),
        }),
    };
    println!("list = {:?}", list);
}

/// requires two non zero integers
/// returns the greatest common denominator
fn gcd(mut n: u64, mut m: u64) -> u64 {
    // panic if conditions are not met
    // idiomatic rust
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            mem::swap(&mut n, &mut m);
        }

        m = m % n;
    }

    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

#[test]
#[should_panic]
fn test_zeros() {
    gcd(0, 15);
}

#[test]
fn test_person() {
    let joe = Person {
        name: String::from("Joe"),
        nick_name: String::from("Joey"),
        age: 55,
        friend: None,
    };

    assert_eq!("Joe", joe.name);
    assert_eq!(55, joe.age);

    // typed destructuring ftw. ignore fields with the dot dot
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

#[test]
fn test_destructuring() {
    let (a, b) = (1, 2);
    assert_eq!(1, a);
    assert_eq!(2, b);
}

#[test]
fn test_tuples() {
    let numbers = (1, 2, 3);
    assert_eq!(2, numbers.1);

    let silly_tuple = (1,);
    assert_eq!(silly_tuple.0, 1);

    // let silly_int = (1); // legal but silly and compiler will cry

    let mut a_tuple = (1, 1.4, false);
    assert_eq!(format!("{:?}", a_tuple), "(1, 1.4, false)");
    // a_tuple = (2, 3, true); // compile time check for a f64. but only if running the tests
    a_tuple = (2, 3f64, true);
    assert_eq!(format!("{:?}", a_tuple), "(2, 3.0, true)");
}

#[test]
fn test_formats() {
    let small = 10.1234f32;
    assert_eq!(format!("small is {:.2}", small), "small is 10.12");
}

#[test]
fn test_borrow() {
    let mut a: u8 = 10;
    let b: u8 = 10;
    let mut the_ref = &a;

    println!(
        "a: {}, b: {}, *the_ref: {} at {:p}",
        a, b, *the_ref, the_ref
    );
    assert_eq!(10, *the_ref);

    // a = 12; // cannot assign to `a` because it is borrowed
    println!(
        "a: {}, b: {}, *the_ref: {} at {:p}",
        a, b, *the_ref, the_ref
    );

    // so we return the borrowed reference
    the_ref = &b;
    a = 12;
    println!(
        "a: {}, b: {}, *the_ref: {} at {:p}",
        a, b, *the_ref, the_ref
    );
    assert_eq!(10, *the_ref);

    the_ref = &a;
    println!(
        "a: {}, b: {}, *the_ref: {} at {:p}",
        a, b, *the_ref, the_ref
    );
    assert_eq!(12, *the_ref);
}

#[test]
fn test_vectors() {
    let first_three = vec![1, 2, 3];
    assert_eq!(6, first_three.iter().sum());
    // first_three.push(4); // not mutable

    let mut numbers: Vec<i32> = (1..3).collect();
    assert_eq!(3, numbers.iter().sum()); // the range does not include the endpoint so we add the 3
    numbers.push(3);
    assert_eq!(6, numbers.iter().sum());
}

#[test]
fn test_slices() {
    let numbers: Vec<i32> = (1..10).collect();
    let slice_of_numbers = &numbers[5..7]; // index 5 and 6
    assert_eq!(45, numbers.iter().sum());
    assert_eq!(6 + 7, slice_of_numbers.iter().sum());
}

#[test]
fn test_mutation() {
    let mut fname = "Fred".to_string();
    fname.push('!');
    assert_eq!("Fred!", fname);

    let ref_to_fname: &String = &fname;
    assert_eq!("Fred!", ref_to_fname);
    assert_eq!("Fred!", fname);

    // mutable reference to a mutable string...
    let mut mutable_ref_to_fname: &mut String = &mut fname;

    // assert_eq!("Fred!", fname); // creating the mutable reference borrows fname

    assert_eq!("Fred!", mutable_ref_to_fname);
    mutable_ref_to_fname.push('!');
    assert_eq!("Fred!!", *mutable_ref_to_fname);

    let mut temp = "Hi".to_string();
    mutable_ref_to_fname = &mut temp;
    assert_eq!("Hi", *mutable_ref_to_fname); // rust tolerates old C devs
    assert_eq!("Hi", mutable_ref_to_fname); // you do not need the dereference

    assert_eq!("Fred!!", fname);
}

#[test]
fn test_if_let() {
    let odds = (1, 3, 5);

    let result = if let (1, middle, _) = odds {
        middle
    } else {
        -1
    };
    assert_eq!(3, result);

    let result = if let (2, _, last) = odds { last } else { -1 };
    assert_eq!(-1, result);
}

#[test]
fn test_match() {
    let odds = (1, 3, 5);

    let result = match odds {
        (1, middle, _) => middle,
        (2, _, last) => last,
        _ => -1,
    };
    assert_eq!(3, result);

    let name_number = ("fred", 12);
    let extracted_number = match name_number {
        (_, number @ 8..=12) => number,
        _ => -1,
    };

    assert_eq!(12, extracted_number);
}

#[test]
fn test_results() {
    fn process(result: Result<u32, core::num::ParseIntError>) -> u32 {
        match result {
            Ok(value) => value,
            Err(_) => 0,
        }
    }

    assert_eq!(23, process("23".parse::<u32>()));
    assert_eq!(0, process("2q3".parse::<u32>()));

    fn shorter_process(result: Result<u32, core::num::ParseIntError>) -> u32 {
        result.unwrap_or(0)
    }

    assert_eq!(23, shorter_process("23".parse::<u32>()));
    assert_eq!(0, shorter_process("2q3".parse::<u32>()));
}

#[test]
fn test_chained_results() {
    fn process_things(raws: &[&str]) -> Result<u32, core::num::ParseIntError> {
        let mut sum = 0;
        for raw in raws {
            sum += raw.parse::<u32>()?;
        }

        Ok(sum)
    }

    fn pointless_indirection(raws: &[&str]) -> Result<u32, core::num::ParseIntError> {
        let result: u32 = process_things(raws)?;
        Ok(result)
    }

    assert_eq!(5, process_things(&["1", "3", "1"]).expect("error"));
    assert!(process_things(&["1", "2", "3q"]).is_err());

    assert_eq!(5, pointless_indirection(&["1", "3", "1"]).expect("error"));
    assert!(pointless_indirection(&["1", "2", "3q"]).is_err());
}

#[test]
fn test_modularity() {
    fn copy_it<T: Copy>(value: &T) -> T {
        *value
    }

    mod things {
        pub struct Thing {
            pub weight: u32,
        }
    }

    mod stuff {
        #[derive(Copy, Clone)]
        pub struct Stuff {
            pub width: u32,
            pub height: u32,
        }
    }

    use stuff::Stuff;
    use things::Thing;
    let my_thing = Thing { weight: 44 };

    let my_stuff = Stuff {
        width: 23,
        height: 11,
    };

    assert_eq!(44, my_thing.weight);

    let more_stuff = copy_it(&my_stuff);
    assert_eq!(23, more_stuff.width);
}

#[test]
fn test_pedants_patience() {
    #[allow(non_snake_case)]
    let CamelCaseVariableExist = true;
    assert_eq!(true, CamelCaseVariableExist);

    // let NotAnotherCamelCaseVariableExist = true; // evil is limited
}

#[test]
fn test_structs() {
    struct Inner {
        favorite_color: String,
    }

    struct Outer {
        favorite_number: u32,
        inner: Inner,
    }

    let outer = Outer {
        favorite_number: 12,
        inner: Inner {
            favorite_color: "red".to_string(),
        },
    };

    assert_eq!(12, outer.favorite_number);
    assert_eq!("red", outer.inner.favorite_color);

    let inside = Inner {
        favorite_color: "blue".to_string(),
    };

    let outside = Outer {
        favorite_number: 22,
        inner: inside,
    };

    assert_eq!(22, outside.favorite_number);
    assert_eq!("blue", outside.inner.favorite_color);

    // assert_eq!("blue", inside.favorite_color); // nope value has been moved
}

#[test]
fn test_traits() {
    trait Consumable {
        fn consume(&self) -> u32;
    }

    #[derive(Copy, Clone)]
    enum Fudge {
        Strawberry = 100,
        Walnut = 120,
    }

    struct Meat {
        quantity: u32,
    }

    impl Consumable for Fudge {
        fn consume(&self) -> u32 {
            *self as u32
        }
    }

    impl Consumable for Meat {
        fn consume(&self) -> u32 {
            self.quantity * 100
        }
    }

    // runtime polymorphism with a vtable
    // dyn is essential for heterogeneous collections
    let consumables: Vec<Box<dyn Consumable>> = vec![
        Box::new(Fudge::Strawberry),
        Box::new(Meat { quantity: 2 }),
        Box::new(Fudge::Walnut),
    ];

    let mut total: u32 = 0;
    for consumable in consumables.iter() {
        total += consumable.consume();
    }

    assert_eq!(420, total);
    assert_eq!(100, Fudge::Strawberry.consume());

    let sum = consumables
        .iter()
        .fold(0, |acc, consumable| acc + consumable.consume());
    assert_eq!(420, sum);

    // generic function gets compiled for each type that is used
    fn calculate_calories<T: Consumable>(consumable: T) -> u32 {
        consumable.consume()
    }
    assert_eq!(120, calculate_calories(Fudge::Walnut));
    assert_eq!(200, calculate_calories(Meat { quantity: 2 }));
}

#[test]
fn test_generic_in_traits() {
    // the bound that T must implement Sized is implied but maybe it makes sense to type it out
    // the compiler assumes itSized
    trait Boxable<T: Sized> {
        fn box_it(value: T) -> Box<T> {
            Box::<T>::new(value)
        }

        fn box_me(&self) -> Box<T>;
    }

    impl Boxable<u32> for u32 {
        fn box_me(&self) -> Box<u32> {
            Box::new(*self)
        }
    }

    #[derive(Copy, Clone)]
    struct Thing(u32, u32);
    struct OtherThing {
        first: u32,
        second: u32,
    }

    impl Boxable<Thing> for Thing {
        fn box_me(&self) -> Box<Thing> {
            Box::new(*self)
        }
    }

    impl Boxable<Thing> for OtherThing {
        fn box_it(value: Thing) -> Box<Thing> {
            Box::new(value)
        }

        fn box_me(&self) -> Box<Thing> {
            Box::new(Thing(self.first, self.second))
        }
    }

    let boxed_int = 32.box_me();
    assert_eq!(32, *boxed_int);

    let boxed_thing = Thing(12, 22).box_me();
    let Thing(_, second) = *boxed_thing;
    assert_eq!(22, second);

    let boxed_from_other_thing = OtherThing {
        first: 11,
        second: 21,
    }
    .box_me();
    assert_eq!(21, boxed_from_other_thing.1);
}

#[test]
fn test_closures() {
    fn repeat<F: FnMut()>(mut closure: F, count: u32) {
        for _ in 0..count {
            closure();
        }
    }

    let mut count = 0;
    repeat(|| count += 1, 5);
    assert_eq!(5, count);

    struct NumberHolder {
        value: u32,
    }

    let numbers = 0..20;
    let even_numbers: Vec<NumberHolder> = numbers
        .filter(|value| value % 2 == 0)
        .map(|value| NumberHolder { value })
        .collect();
    assert_eq!(2, even_numbers[1].value);
}

#[test]
fn test_fibonacci_iterator() {
    #[derive(Debug)]
    struct FibonacciIterator {
        previous: u32,
        current: u32,
    }

    impl FibonacciIterator {
        fn new() -> Self {
            FibonacciIterator {
                previous: 0,
                current: 0,
            }
        }
    }

    impl Iterator for FibonacciIterator {
        type Item = u32;

        // 0 0 -> 0
        // 0 1 -> 1
        // 1 1 -> 1

        fn next(&mut self) -> Option<<Self as Iterator>::Item> {
            match self.current {
                0 => {
                    self.current = 1;
                    Some(0u32)
                }
                _ => {
                    let temp = self.current;
                    self.current = self.previous + self.current;
                    self.previous = temp;
                    Some(self.previous)
                }
            }
        }
    }

    let mut fib = FibonacciIterator::new();
    assert_eq!(0, fib.current);

    assert_eq!(Some(0u32), fib.next());
    assert_eq!(Some(1u32), fib.next());
    assert_eq!(Some(1u32), fib.next());
    assert_eq!(Some(2u32), fib.next());
    assert_eq!(Some(3u32), fib.next());
    assert_eq!(Some(5u32), fib.next());
    assert_eq!(Some(8u32), fib.next());
}

use std::mem;

mod people {
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
}

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

use crate::people::Person;

fn main() {
    println!("Hello, world!");
    println!("gcd = {}", gcd(12, 14));

    let mut fred = Person {
        name: String::from("Fred"),
        nick_name: String::from("Freddy"),
        age: 15,
    };

    println!("fred = {:?}", fred);
    fred.increase_age();
    println!("fred = {:?}", fred);
    // fred.age += 1; // nope private

    println!("move = {}", Move::Up(2).extract_motion());
    println!("move = {}", Move::Down(2).extract_motion());
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
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
fn test_moves() {
    let up2 = Move::Up(2);
    let down3 = Move::Down(3);

    assert_eq!(2, up2.extract_motion());
    assert_eq!(-3, down3.extract_motion());
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
    };

    let jane_name = jane.name;
    assert_eq!("Jane", jane_name);
    // assert_eq!("Jane", jane.name); // can not access jane.name since it moved
    assert_eq!(54, jane.age);
    assert_eq!("Janey", jane.nick_name);
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

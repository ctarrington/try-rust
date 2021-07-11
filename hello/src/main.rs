use std::mem;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    println!("Hello, world!");
    println!("gcd = {}", gcd(12, 14));

    let fred = Person {
        name: String::from("Fred"),
        age: 15,
    };

    println!("fred = {:?}", fred);
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
        age: 55,
    };

    assert_eq!("Joe", joe.name);
    assert_eq!(55, joe.age);

    // typed destructuring ftw
    let Person {
        name: mut fname,
        age: _,
    } = joe;

    // assert_eq!("Joe", joe.name); // can not access joe anymore since it moved

    assert_eq!("Joe", fname);
    fname = fname.to_uppercase();
    assert_eq!("JOE", fname);
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

use std::mem;

fn main() {
    println!("Hello, world!");
    println!("gcd = {}", gcd(12, 14));
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
fn test_destructuring() {
    let (a, b) = (1, 2);
    assert_eq!(1, a);
    assert_eq!(2, b);
}

#[test]
fn test_tuples() {
    let numbers = (1, 2, 3);
    assert_eq!(2, numbers.1);
}

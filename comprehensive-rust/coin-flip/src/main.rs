use rand::Rng;
use crate::CoinFlip::{Heads, Tails};

#[derive(Debug)]
enum CoinFlip {
    Heads,
    Tails,
}

impl CoinFlip {
    fn flip() -> CoinFlip {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(0..10000);
        return if value % 2 == 0 {
            Heads
        } else {
            Tails
        }
    }
}

#[test]
fn test_fair_flips() {
    const FLIP_COUNT: i32 = 1_000_000;
    let mut flips = Vec::new();
    for _ in 0..FLIP_COUNT {
        flips.push(CoinFlip::flip());
    }

    let mut sum = 0;
    for index in 0..flips.len() {
        sum += match flips[index] {
            Heads => 1,
            Tails => 0,
        };
    }

    let sum: f64 = sum.into();
    let flip_count: f64 = FLIP_COUNT.into();
    let ratio = sum / flip_count;
    let delta: f64 = ratio - 0.5;
    let delta = delta.abs();
    assert!(delta.abs() < 0.01);
}

fn main() {
    println!("flip: {:#?}", CoinFlip::flip());
}

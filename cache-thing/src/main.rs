use std::time;
use rand::prelude::*;
use rayon::prelude::*;

fn main() {
    println!("Hello, world!");

    let mut a_values: Vec<u32> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ctr in 0..1000000 {
        a_values.push(rng.gen());
    }

    let begin = time::Instant::now();
    a_values.par_sort_unstable();
    let elapsed = time::Instant::now() - begin;

    println!("elapsed w rayon: {:?}", elapsed);
}

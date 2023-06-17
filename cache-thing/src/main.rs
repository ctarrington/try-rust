use std::time;
use rand::prelude::*;
use rayon::prelude::*;

fn main() {
    println!("Hello, world!");

    let mut a_values: Vec<f32> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ctr in 0..1000000 {
        a_values.push(rng.gen());
    }

    let begin = time::Instant::now();
    a_values.par_sort_by(|left, right| left.partial_cmp(right).unwrap());
    let elapsed = time::Instant::now() - begin;

    println!("first few {:?}", &a_values[0..4]);
    println!("elapsed w rayon: {:?}", elapsed);
}

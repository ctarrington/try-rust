use rand::Rng;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{cmp, thread, time};

/// simplistic proof of work scheme to introduce a little variability how long a process takes
/// picks a random number until it is at or below a target
fn calculate_proof_of_work(target: u32, range_max: u32) -> (u32, Duration) {
    let mut rng = rand::thread_rng();

    let begin = time::Instant::now();
    let found = loop {
        let value: u32 = rng.gen_range(0..range_max);
        if value <= target {
            break value;
        }
    };
    let elapsed = time::Instant::now() - begin;

    (found, elapsed)
}

#[derive(Debug)]
struct Thing {
    proof: (u32, Duration),
}

impl Thing {
    fn new() -> Self {
        Thing {
            proof: (0, Duration::from_secs(0)),
        }
    }

    fn tick(&mut self) {
        let (value, elapsed) = calculate_proof_of_work(10, 10_000);
        self.proof = (self.proof.0 + value, self.proof.1 + elapsed);
    }
}

/// make blocks for the elements, they may be slightly different sizes and for small
/// sizes the last block may not be created
fn calculate_execution_blocks(size: usize, thread_count: usize) -> Vec<(usize, usize)> {
    let mut execution_blocks = vec![];

    let stop = size - 1;
    let raw_stride = (size as f32) / thread_count as f32;
    let stride = raw_stride.ceil() as usize;

    let mut start_index = 0;
    for _ in 0..thread_count {
        let stop_index = start_index + stride;
        let stop_index = cmp::min(stop_index, stop);
        if stop_index >= start_index {
            execution_blocks.push((start_index, stop_index));
        }
        start_index = start_index + stride + 1;
    }
    execution_blocks
}

fn main() {
    let things: Vec<Mutex<Thing>> = (0..10).map(|_| Mutex::new(Thing::new())).collect();
    let things = Arc::new(things);

    println!("before things {:?}", things);
    let blocks = calculate_execution_blocks(things.len(), 3);
    println!("blocks: {:?} ", blocks);

    let mut handles = vec![];
    blocks.iter().for_each(|block| {
        let (start_index, stop_index) = (block.0, block.1);
        let things_for_threads = things.clone();

        handles.push(thread::spawn(move || {
            let slice_of_things = &things_for_threads[start_index..=stop_index];
            for thing in slice_of_things.iter() {
                thing.lock().expect("unable to lock thing").tick();
            }
        }));
    });

    for handle in handles {
        handle.join().expect("unable to join tick handle");
    }

    println!("\nafter things {:?}", things);
}

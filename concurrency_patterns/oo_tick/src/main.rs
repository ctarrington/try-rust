use rand::Rng;
use std::sync::{mpsc, Arc, Mutex};
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

fn tick_concurrent(count: usize, thread_count: usize) {
    let things: Vec<Mutex<Thing>> = (0..count).map(|_| Mutex::new(Thing::new())).collect();
    let things = Arc::new(things);

    let blocks = calculate_execution_blocks(things.len(), thread_count);
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

    println!(
        "\nconcurrent thing 0 is {:?} len: {}",
        things.get(0),
        things.len()
    );

    let sum: u32 = things
        .iter()
        .map(|thing| thing.lock().expect("unable to lock thing").proof.0)
        .sum();
    let average = sum as f32 / things.len() as f32;
    println!("sum: {}, average: {}", sum, average);
}

fn tick_single(count: usize) {
    let mut things: Vec<Thing> = (0..count).map(|_| Thing::new()).collect();
    for thing in &mut things {
        thing.tick();
    }

    println!(
        "\nsingle thing 0 is {:?} len: {}",
        things.get(0),
        things.len()
    );

    let sum: u32 = things.iter().map(|thing| thing.proof.0).sum();
    let average = sum as f32 / things.len() as f32;
    println!("sum: {}, average: {}", sum, average);
}

fn tick_pipeline(count: usize, thread_count: usize) {
    let (thing_sender, thing_receiver) = mpsc::channel::<Thing>();
    let (collation_sender, collation_receiver) = mpsc::channel::<Vec<Thing>>();

    let blocks = calculate_execution_blocks(count, thread_count);
    for (start, stop) in blocks {
        let thing_sender = thing_sender.clone();
        thread::spawn(move || {
            let things: Vec<Thing> = (start..=stop).map(|_| Thing::new()).collect();
            for mut thing in things {
                thing.tick();
                thing_sender.send(thing).expect("error sending a thing");
            }
        });
    }

    let collation_handler = thread::spawn(move || {
        let mut received_things: Vec<Thing> = vec![];
        for thing in thing_receiver {
            received_things.push(thing);
            if received_things.len() == count {
                break;
            }
        }

        collation_sender.send(received_things).unwrap();
    });

    let collated_things = collation_receiver
        .recv()
        .expect("unable to receive the collated things");
    println!(
        "\ncollated thing 0 is {:?} len: {}",
        collated_things.get(0),
        collated_things.len()
    );

    let sum: u32 = collated_things.iter().map(|thing| thing.proof.0).sum();
    let average = sum as f32 / collated_things.len() as f32;
    println!("sum: {}, average: {}", sum, average);

    // note we only join on the final receiver thread
    collation_handler
        .join()
        .expect("unable to join the collation handler");
}

fn main() {
    let thing_count = 100_000;
    let thread_count = 4;

    let begin = time::Instant::now();
    tick_concurrent(thing_count, thread_count);
    let elapsed_concurrent = time::Instant::now() - begin;

    let begin = time::Instant::now();
    tick_single(thing_count);
    let elapsed_single = time::Instant::now() - begin;

    let begin = time::Instant::now();
    tick_pipeline(thing_count, thread_count);
    let elapsed_pipeline = time::Instant::now() - begin;

    println!("\nsingle: {:?}", elapsed_single);

    let ratio = elapsed_single.as_micros() as f32 / elapsed_concurrent.as_micros() as f32;
    println!("concurrent: {:?}, ratio: {}", elapsed_concurrent, ratio);

    let ratio = elapsed_single.as_micros() as f32 / elapsed_pipeline.as_micros() as f32;
    println!("pipeline: {:?}, ratio: {}", elapsed_pipeline, ratio);
}

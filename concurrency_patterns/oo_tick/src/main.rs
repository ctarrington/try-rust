use std::sync::{Arc, Mutex};
use std::{cmp, thread};

#[derive(Debug)]
struct Thing {
    value: u32,
}

impl Thing {
    fn new() -> Self {
        Thing { value: 0 }
    }

    fn tick(&mut self) {
        self.value += 1;
    }
}

/// make blocks for the elements, they may be slightly different sizes and for small
/// sizes the last block may not be created
fn calculate_execution_blocks(size: usize, thread_count: usize) -> Vec<(usize, usize)> {
    let mut execution_blocks = Vec::new();

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
    let things: Vec<Arc<Mutex<Thing>>> = (0..10)
        .map(|_| Arc::new(Mutex::new(Thing::new())))
        .collect();

    println!("before things {:?}", things);
    let blocks = calculate_execution_blocks(things.len(), 3);
    println!("blocks: {:?} ", blocks);

    let mut handles = Vec::new();
    let things = Arc::new(things);

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

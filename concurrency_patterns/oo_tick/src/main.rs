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

/// make slices for the elements, the slices may be slightly different sizes and for small
/// elements sizes the last slice may not be created
fn calculate_execution_slices<T>(
    elements: &Vec<Arc<Mutex<T>>>,
    thread_count: usize,
) -> Vec<(usize, usize)> {
    let mut execution_slices = Vec::new();

    let stop = elements.len() - 1;
    let raw_stride = (elements.len() as f32) / thread_count as f32;
    let stride = raw_stride.ceil() as usize + 1;

    let mut start_index = 0;
    for _ in 0..thread_count {
        let stop_index = start_index + stride - 1;
        let stop_index = cmp::min(stop_index, stop);
        if stop_index >= start_index {
            execution_slices.push((start_index, stop_index));
        }
        start_index += stride;
    }
    execution_slices
}

fn main() {
    let things: Vec<Arc<Mutex<Thing>>> = (0..10)
        .map(|_| Arc::new(Mutex::new(Thing::new())))
        .collect();
    let blocks = calculate_execution_slices(&things, 3);
    println!("blocks: {:?} ", blocks);

    let mut handles = Vec::new();
    let things = Arc::new(things);

    blocks.iter().for_each(|block| {
        let (start_index, stop_index) = block;
        let start_index = *start_index;
        let stop_index = *stop_index;
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

    println!("after things {:?}", things);
}

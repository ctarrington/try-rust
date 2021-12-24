use std::{cmp, thread, };
use std::sync::{Arc, Mutex};

/// make slices for the elements, the slices may be slightly different sizes and for small
/// elements sizes the last slice may not be created
fn calculate_execution_slices<'a, T>(elements: &'a Vec<T>, thread_count: usize) -> Vec<&'a [T]> {
    let mut execution_slices = Vec::new();

    let stop = elements.len() - 1;
    let raw_stride = (elements.len() as f32) / thread_count as f32;
    let stride = raw_stride.ceil() as usize + 1;

    let mut start_index = 0;
    for _ in 0..thread_count {
        let stop_index = start_index + stride - 1;
        let stop_index = cmp::min(stop_index, stop);
        let slice = &elements[start_index..=stop_index];
        if slice.len() > 0 {
            execution_slices.push(slice);
        }
        start_index += stride;
    }
    execution_slices
}

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

fn main() {
    let things_for_thread = Arc::new(vec![Mutex::new(Thing::new()), Mutex::new(Thing::new())]);
    let things = things_for_thread.clone();
    println!("before things {:?}", things);

    let mut handles = Vec::new();
    handles.push(thread::spawn(move || {
        for thing in things_for_thread.iter() {
            thing.lock().expect("unable to unlock thing").tick();
        }
    })
    );

    for handle in handles {
        handle.join().expect("unable to join thing tick handle");
    }
    println!("after things {:?}", things);


//    let things: Vec<Thing> = (0..20).map(|_| Thing::new()).collect();
//   let slices = calculate_execution_slices(&things, 3);
//  println!("slices: {:?} ", slices);
}

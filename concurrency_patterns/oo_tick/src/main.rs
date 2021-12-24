use std::{cmp, thread, };
use std::ops::DerefMut;
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
    println!("Hello, world!!");
    let thing1_for_thread = Arc::new(Mutex::new(Thing::new()));
    let thing1 = thing1_for_thread.clone();
    println!("before thing1 {:?}", thing1);

    let thing2_for_thread = Arc::new(Mutex::new(Thing::new()));
    let thing2 = thing2_for_thread.clone();
    println!("before thing2 {:?}", thing2);

    let mut handles = Vec::new();
    handles.push(thread::spawn(move || {
        let thing = &mut thing1_for_thread.lock().expect("unable to get the lock on the thing");
        let thing = thing.deref_mut();
        thing.tick();
    })
    );

    handles.push(thread::spawn(move || {
        let thing = &mut thing2_for_thread.lock().expect("unable to get the lock on the thing");
        let thing = thing.deref_mut();
        thing.tick();
    })
    );

    for handle in handles {
        handle.join().expect("unable to join thing tick handle");
    }
    println!("after thing1 {:?}", thing1);
    println!("after thing2 {:?}", thing2);


    let things: Vec<Thing> = (0..20).map(|_| Thing::new()).collect();
    let slices = calculate_execution_slices(&things, 3);
    println!("slices: {:?} ", slices);
}

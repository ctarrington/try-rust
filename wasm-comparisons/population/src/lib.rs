mod utils;

use rand::prelude::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, population!");
}

#[wasm_bindgen]
pub fn mean(count: u16) -> f32 {
    let mut sum: f32 = 0.0;
    for _ in 0..count {
        let value: f32 = random();
        sum += value;
    }
    sum / f32::from(count)
}

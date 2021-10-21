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
pub fn random_mean(count: u16) -> f32 {
    let mut sum: f32 = 0.0;
    for _ in 0..count {
        let value: f32 = random();
        sum += value;
    }
    sum / f32::from(count)
}

struct LCG {
    current: f32,
    a: f32,
    c: f32,
    m: f32,
}

impl LCG {
    fn next(&mut self) -> f32 {
        self.current = (self.a * self.current + self.c) % self.m;
        self.current / self.m
    }
}

fn lcg(seed: f32) -> LCG {
    LCG {
        current: seed,
        a: 8121.0,
        c: 28411.0,
        m: 134456.0,
    }
}

#[wasm_bindgen]
pub fn lcg_mean(count: u16) -> f32 {
    let mut the_lcg = lcg(123456789.0);
    let mut sum: f32 = 0.0;
    for _ in 0..count {
        let value: f32 = the_lcg.next();
        sum += value;
    }
    sum / f32::from(count)
}

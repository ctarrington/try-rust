mod utils;

use std::convert::TryInto;
use std::fmt;

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

const WIDTH: i32 = 128;
const HEIGHT: i32 = 128;
const CELL_COUNT: usize = WIDTH as usize * HEIGHT as usize;

fn get_index(row: usize, column: usize) -> usize {
    (row * WIDTH as usize + column) as usize
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    current_index: usize,
    previous_index: usize,
    cells_list: [[Cell; CELL_COUNT]; 2],
    image_bytes: [u8; CELL_COUNT * 4],
}

#[wasm_bindgen]
impl Universe {
    fn get_value(&self, row: i32, column: i32) -> u32 {
        let mut adjusted_row = row;
        let mut adjusted_column = column;

        if row < 0 {
            adjusted_row = HEIGHT - 1;
        }

        if row >= HEIGHT {
            adjusted_row = 0;
        }

        if column < 0 {
            adjusted_column = HEIGHT - 1;
        }

        if column >= HEIGHT {
            adjusted_column = 0;
        }

        self.cells_list[self.previous_index][get_index(
            adjusted_row.try_into().unwrap(),
            adjusted_column.try_into().unwrap(),
        )] as u32
    }

    fn live_neighbor_count(&self, row: i32, column: i32) -> u32 {
        let mut count = 0;
        for ri in row - 1..=row + 1 {
            for ci in column - 1..=column + 1 {
                if ri != row || ci != column {
                    count += self.get_value(ri, ci);
                }
            }
        }

        count
    }

    pub fn tick(&mut self) {
        self.previous_index = self.current_index;
        self.current_index = (self.current_index + 1) % 2;

        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                let idx = get_index(row.try_into().unwrap(), col.try_into().unwrap());

                let new_value = match (
                    self.cells_list[self.previous_index][idx],
                    self.live_neighbor_count(row, col),
                ) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };
                self.cells_list[self.current_index][idx] = new_value;
            }
        }
    }

    pub fn new() -> Universe {
        let mut cells = [Cell::Dead; CELL_COUNT];

        for index in 0..CELL_COUNT {
            if index % 2 == 0 || index % 7 == 0 {
                cells[index] = Cell::Alive;
            }
        }

        Universe {
            previous_index: 1,
            current_index: 0,
            cells_list: [cells, [Cell::Dead; CELL_COUNT]],
            image_bytes: [0; CELL_COUNT * 4],
        }
    }

    pub fn render_as_text(&self) -> String {
        self.to_string()
    }

    pub fn render_to_image(&mut self) -> *const u8 {
        for byte_index in 0..CELL_COUNT * 4 {
            let cell_index = byte_index / 4 as usize;
            let cell = self.cells_list[self.current_index][cell_index];
            let red = if cell == Cell::Dead { 0 } else { 255 };

            match byte_index % 4 {
                3 => self.image_bytes[byte_index] = red,
                2 => self.image_bytes[byte_index] = 0,
                1 => self.image_bytes[byte_index] = 0,
                0 => self.image_bytes[byte_index] = 255,
                _ => {}
            };
        }
        self.image_bytes.as_ptr()
    }

    pub fn width(&self) -> i32 {
        WIDTH
    }

    pub fn height(&self) -> i32 {
        HEIGHT
    }

    pub fn cells(&self) -> *const Cell {
        self.cells_list[self.current_index].as_ptr()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                let cell = self.cells_list[self.current_index]
                    [get_index(row.try_into().unwrap(), col.try_into().unwrap())];
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

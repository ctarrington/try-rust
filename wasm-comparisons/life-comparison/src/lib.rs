mod utils;

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

const WIDTH: usize = 128;
const HEIGHT: usize = 128;

fn get_index(row: usize, column: usize) -> usize {
    (row * WIDTH + column) as usize
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
    width: usize,
    height: usize,
    current_index: usize,
    previous_index: usize,
    cells_list: [[Cell; WIDTH * HEIGHT]; 2],
}

#[wasm_bindgen]
impl Universe {
    fn get_value(&self, row: usize, column: usize) -> u32 {
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

        self.cells_list[self.previous_index][get_index(adjusted_row, adjusted_column)] as u32
    }

    fn live_neighbor_count(&self, row: usize, column: usize) -> u32 {
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

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = get_index(row, col);

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
        let mut cells = [Cell::Dead; WIDTH * HEIGHT];

        for index in 0..WIDTH * HEIGHT {
            if index % 2 == 0 || index % 7 == 0 {
                cells[index] = Cell::Alive;
            }
        }

        Universe {
            width: WIDTH,
            height: HEIGHT,
            previous_index: 1,
            current_index: 0,
            cells_list: [cells, [Cell::Dead; WIDTH * HEIGHT]],
        }
    }

    pub fn render_as_text(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells_list[self.current_index].as_ptr()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let cell = self.cells_list[self.current_index][get_index(row, col)];
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

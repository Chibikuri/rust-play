mod utils;
use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Life game
#[wasm_bindgen]
#[repr(u8)] // to point out cell is represented as single byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

// prepare implementation for Universe to access its variables
#[wasm_bindgen]
impl Universe {
    // constructor
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Universe {
            width: width,
            height: height,
            cells: cells,
        }
    }
    // public implementation for bridging Javascript
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neigbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule1: live cell with fewer than two live neighbours -> under population (die)
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule2: live cell with 2 or three live neighbours -> next generation (alive)
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule3: live cell with more than three neighbours -> over population (die)
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule4: dead cell with exact three neighbors -> reproduction (become alive)
                    (Cell::Dead, 3) => Cell::Alive,
                    // otherwise, just the same as previous one.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    // usize can be identifier of a cell
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // count how many cells are alive around the node
    fn live_neigbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '□' } else { '■' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

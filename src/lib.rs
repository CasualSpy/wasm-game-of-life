mod utils;

use wasm_bindgen::prelude::*;
extern crate js_sys;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;
extern crate web_sys;
use web_sys::console;

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer {name}
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
    // ( $( $t:tt )* ) => {
        // web_sys::console::log_1(&format!( $( $t )* ).into());
    // }
// }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width:u32,
    height:u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row:u32, column:u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row:u32, col:u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

        let west = if col == 0 {
            self.width - 1
        } else { 
            col - 1
        };

        let east = if col == self.width - 1 {
            0
        } else {
            col + 1
        };

        

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;
        let n = self.get_index(north, col);
        count += self.cells[n] as u8;
        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;
        let w = self.get_index(row, west);
        count += self.cells[w] as u8;
        let e = self.get_index(row, east);
        count += self.cells[e] as u8;
        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;
        let s = self.get_index(south, col);
        count += self.cells[s] as u8;
        let se = self.get_index(south, east);
        count += self.cells[se] as u8;
        count
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn tick(&mut self) {
        // let _timer = Timer::new("Universe::tick");
        let mut next = {
            // let _timer = Timer::new("allocate next cells");
            self.cells.clone()
        };

        {
            // let _timer = Timer::new("new generation");
            for row in 0..self.height {
                for col in 0..self.width{
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);

                    next.set(idx, match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise
                    });
                }
            }
        }

        // let _timer = Timer::new("free old cells");
        self.cells = next;
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.set(idx, !self.cells[idx]);
    }

    pub fn kill_all(&mut self) {
        for i in 0..self.width * self.height {
            self.cells.set(i as usize, false);
        }
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 128;
        let height = 128;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        
        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.5);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        for i in 0..width * self.height {
            self.cells.set(i as usize, false);
        }
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        for i in 0..self.width * height {
            self.cells.set(i as usize, false);
        }
    }
}

impl Universe {
    pub fn get_cells(&self) {
        &self.cells;
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}
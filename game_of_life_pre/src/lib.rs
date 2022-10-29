use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::console;

// 1. We start our type definition for cells
//
// It is important that we have #[repr(u8)],
// so that each cell is represented as a single byte.
// It is also important that the Dead variant is 0
// and that the Alive variant is 1,
// so that we can easily count a cell's live neighbors with addition.

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

// 2. Next Next, let's define the universe.
//
// The universe has a width and a height,
// and a vector of cells of length width * height.

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

// 3. To access the cell at a given row and column,
// we translate the row and column into an index into the cells vector, as described earlier:

#[wasm_bindgen] // Public methods, exported to JavaScript.
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // 4. In order to calculate the next state of a cell,
    // we need to get a count of how many of its neighbors are alive.
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
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

    // 5. Now we have everything we need to compute the next generation from the current one!
    // Each of the Game's rules follows a straightforward translation into a condition on a match expression.
    // Additionally, because we want JavaScript to control when ticks happen,
    // we will put this method inside a #[wasm_bindgen] block,
    // so that it gets exposed to JavaScript.
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
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

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    } //^--fn tick

    // 7_. Finally, we define a constructor that initializes the universe
    // with an interesting pattern of live and dead cells,
    // as well as a render method:
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|| { // |i|
                if js_sys::Math::random() < 0.5 { //i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    } //^--fn new

    pub fn render(&self) -> String {
        self.to_string()
    }

} //^--impl Universe

// 6. So far, the state of the universe is represented as a vector of cells.
// To make this human readable, let's implement a basic text renderer.
// The idea is to write the universe line by line as text,
// and for each cell that is alive, print the Unicode character ◼ ("black medium square").
// For dead cells, we'll print ◻ (a "white medium square").

// By implementing the Display trait from Rust's standard library,
// we can add a way to format a structure in a user-facing manner.
// This will also automatically give us a to_string method.

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

// 7. see in impl Universe 7_.

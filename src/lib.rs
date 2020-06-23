mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Alive = 1,
    Dead = 0,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
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
            width,
            height,
            cells,
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let live_count = self.get_live_neighbor_count(row, col);
                let cell_type = self.cells[index];

                print!("({},{}) live_count: {} ", row, col, live_count);
                if cell_type == Cell::Alive {
                    if live_count < 2 {
                        print!("Alive -> Dead");
                        next[index] = Cell::Dead;
                    }

                    if live_count > 3 {
                        print!("Alive -> Dead");
                        next[index] = Cell::Dead;
                    }
                } else {
                    if live_count == 3 {
                        print!("Dead -> Alive");
                        next[index] = Cell::Alive;
                    }
                }
            }
        }

        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    /// Gets the total live cells around the given cell in the universe grid
    /// The grid wraps at the edges, so some modulo math is used to calculate the correct positions
    fn get_live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let nw = self.get_index(
            (row + self.height - 1) % self.height,
            (col + self.width - 1) % self.width,
        );
        let n = self.get_index((row + self.height - 1) % self.height, col);
        let ne = self.get_index(
            (row + self.height - 1) % self.height,
            (col + 1) % self.width,
        );
        let w = self.get_index(row, (col + self.width - 1) % self.width);
        let e = self.get_index(row, (col + 1) % self.width);
        let sw = self.get_index((row + 1) % self.height, (col + self.width - 1) % self.width);
        let s = self.get_index((row + 1) % self.height, col);
        let se = self.get_index((row + 1) % self.height, (col + 1) % self.width);

        self.cells[nw] as u8
            + self.cells[n] as u8
            + self.cells[ne] as u8
            + self.cells[w] as u8
            + self.cells[e] as u8
            + self.cells[sw] as u8
            + self.cells[s] as u8
            + self.cells[se] as u8
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.chunks(self.width as usize) {
            for &cell in line {
                if cell == Cell::Alive {
                    write!(f, "◼");
                } else {
                    write!(f, "◻");
                }
            }
            write!(f, "\n");
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_live_neighbor_count() {
        use Cell::*;

        let universe = Universe {
            width: 4,
            height: 4,
            cells: vec![
                Alive, Alive, Alive, Alive, Alive, Alive, Alive, Alive, Alive, Alive, Alive, Alive,
                Dead, Dead, Dead, Dead,
            ],
        };

        let count_one = universe.get_live_neighbor_count(2, 1);
        let count_two = universe.get_live_neighbor_count(3, 0);
        let count_three = universe.get_live_neighbor_count(1, 0);

        assert_eq!(count_one, 5);
        assert_eq!(count_two, 6);
        assert_eq!(count_three, 8);
    }

    #[test]
    fn test_tick() {
        use Cell::*;

        let mut universe = Universe {
            width: 4,
            height: 4,
            cells: vec![
                Dead, Alive, Dead, Dead, Dead, Alive, Dead, Dead, Dead, Alive, Dead, Dead, Dead,
                Dead, Dead, Dead,
            ],
        };

        print!("{}", universe.to_string());

        universe.tick();

        print!("{}", universe.to_string());
    }
}

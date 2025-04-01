mod rng;

use crate::cell::Cell;
use rand::SeedableRng;
pub use rng::RngSettings;

use ahash::AHasher;
use std::hash::Hasher;

#[derive(Debug)]
pub struct Grid {
    pub dimension: (i32, i32),
    pub wrapped: bool,
    pub rng_settings: Option<RngSettings>,
    pub lattice: Vec<Cell>,
}

impl Grid {
    pub fn new(dimension: (i32, i32), wrapped: bool, rng_settings: Option<RngSettings>) -> Self {
        let total_cells = dimension.0 * dimension.1;
        let lattice: Vec<Cell> = match &rng_settings {
            Some(rng_settings) => {
                let rng = rand::rngs::StdRng::seed_from_u64(rng_settings.seed);
                let threshold = rng_settings.cooperator_frequency;
                (0..total_cells)
                    .map(|_| {
                        let random_value = rand::Rng::r#gen::<f64>(&mut rng.clone());
                        Cell::new(random_value < threshold)
                    })
                    .collect()
            }
            None => {
                println!("no rng settings detected, defaulting to lone defector");
                let center_index = total_cells / 2;
                (0..total_cells)
                    .map(|i| Cell::new(i != center_index))
                    .collect()
            }
        };

        Grid {
            dimension,
            wrapped,
            rng_settings,
            lattice,
        }
    }

    fn get_index(&self, row: i32, col: i32) -> Option<usize> {
        let num_rows = self.dimension.0;
        let num_cols = self.dimension.1;

        if self.wrapped {
            let wrapped_row = ((row % num_rows) + num_rows) % num_rows;
            let wrapped_col = ((col % num_cols) + num_cols) % num_cols;

            Some((wrapped_row * num_cols + wrapped_col) as usize)
        } else if row >= 0 && row < num_rows && col >= 0 && col < num_cols {
            Some((row * num_cols + col) as usize)
        } else {
            None
        }
    }

    #[inline]
    pub fn get_cell(&self, row: i32, col: i32) -> Option<&Cell> {
        self.get_index(row, col)
            .and_then(|index| self.lattice.get(index))
    }

    #[inline]
    pub fn get_cell_mut(&mut self, row: i32, col: i32) -> Option<&mut Cell> {
        self.get_index(row, col)
            .and_then(|index| self.lattice.get_mut(index))
    }

    pub fn get_lattice_hash(&self) -> u64 {
        let mut hasher = AHasher::default();

        for cell in &self.lattice {
            let value = match cell {
                Cell::CC(_) => 0b00,
                Cell::CD(_) => 0b01,
                Cell::DD(_) => 0b10,
                Cell::DC(_) => 0b11,
            };
            hasher.write_u8(value);
        }

        hasher.finish()
    }
}

#[cfg(test)]
mod tests;

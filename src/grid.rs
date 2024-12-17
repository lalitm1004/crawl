use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::cell::Cell;

#[derive(Debug, PartialEq)]
pub struct RngSettings {
    pub seed: u64,
    pub initial_cooperators: f64,
}

#[derive(Debug, PartialEq)]
pub struct Grid {
    pub num_rows: usize,
    pub num_cols: usize,
    pub wrapped: bool,
    pub rng_settings: Option<RngSettings>,
    pub lattice: Vec<Cell>,
}

impl Grid {
    pub fn new(
        num_rows: usize,
        num_cols: usize,
        wrapped: bool,
        rng_settings: Option<RngSettings>
    ) -> Self {
        let lattice: Vec<Cell> = match &rng_settings {
            Some(rng_settings) => {
                let mut rng = StdRng::seed_from_u64(rng_settings.seed);
                let threshold = rng_settings.initial_cooperators;
                (0..(num_rows*num_cols))
                    .map(|_| Cell::new(rng.gen::<f64>() < threshold))
                    .collect()
            },
            None => {
                println!("No rng settings detected, defaulting to lone defector");
                (0..(num_rows*num_cols))
                    .map(|i| {
                        let is_center: bool = i == num_rows * num_cols / 2;
                        Cell::new(!(is_center))
                    })
                    .collect()
            }
        };

        Grid {
            num_rows,
            num_cols,
            wrapped,
            rng_settings,
            lattice,
        }
    }

    fn get_index(&self, row: i32, col: i32) -> Option<usize> {
        let num_rows= self.num_rows as i32;
        let num_cols= self.num_cols as i32;
        if self.wrapped {
            let wrapped_row = ((row % num_rows) + num_rows) % num_rows;
            let wrapped_col = ((col % num_cols) + num_cols) % num_cols;

            Some((wrapped_row * num_cols + wrapped_col) as usize)
        } else {
            if row >= 0 && row < self.num_rows as i32 && col >= 0 && col < self.num_cols as i32 {
                Some((row * self.num_cols as i32 + col) as usize)
            } else {
                None
            }
        }
    }

    pub fn get_cell(&self, row: i32, col: i32) -> Option<&Cell> {
        let index = Grid::get_index(&self, row, col);
        if let Some(index) = index {
            self.lattice.get(index)
        } else {
            None
        }
    }

    pub fn get_cell_mut(&mut self, row: i32, col: i32) -> Option<&mut Cell> {
        let index = Grid::get_index(&self, row, col);
        if let Some(index) = index {
            self.lattice.get_mut(index)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Grid, RngSettings};
    use crate::cell::Cell;

    #[test]
    fn test_random_grid() {
        let rng_settings = RngSettings {
            seed: 1212901292192,
            initial_cooperators: 0.5,
        };

        let num_rows = 5;
        let num_cols = 5;
        let wrapped = false;
        let grid = Grid::new(num_rows, num_cols, wrapped, Some(rng_settings));

        let expected = vec![
            Cell::new(true) , Cell::new(false), Cell::new(false), Cell::new(false), Cell::new(true),
            Cell::new(true) , Cell::new(false), Cell::new(false), Cell::new(true) , Cell::new(true),
            Cell::new(false), Cell::new(false), Cell::new(true) , Cell::new(true) , Cell::new(true),
            Cell::new(true) , Cell::new(true) , Cell::new(false), Cell::new(true) , Cell::new(false),
            Cell::new(true) , Cell::new(false), Cell::new(true) , Cell::new(true) , Cell::new(true),
        ];

        assert_eq!(grid.lattice, expected);
    }

    #[test]
    fn test_preset_grid() {
        let num_rows = 5;
        let num_cols = 5;
        let wrapped = false;
        let grid = Grid::new(num_rows, num_cols, wrapped, None);

        let expected = vec![
            Cell::new(true), Cell::new(true), Cell::new(true) , Cell::new(true), Cell::new(true),
            Cell::new(true), Cell::new(true), Cell::new(true) , Cell::new(true), Cell::new(true),
            Cell::new(true), Cell::new(true), Cell::new(false), Cell::new(true), Cell::new(true),
            Cell::new(true), Cell::new(true), Cell::new(true) , Cell::new(true), Cell::new(true),
            Cell::new(true), Cell::new(true), Cell::new(true) , Cell::new(true), Cell::new(true),
        ];

        assert_eq!(grid.lattice, expected, "Grid should be lone defector based is RngSettings is None");
    }

    #[test]
    fn test_get_index_non_wrapped() {
        let grid = Grid::new(5, 5, false, None);

        assert_eq!(grid.get_index(0, 0), Some(0), "Non-Wrapped index (0, 0) should return 0");
        assert_eq!(grid.get_index(4, 4), Some(24), "Non-Wrapped index (4, 4) should return 24");
        assert_eq!(grid.get_index(-1, -1), None, "Index out of bounds should return None");
        assert_eq!(grid.get_index(10, 10), None, "Index out of bounds should return None");
    }

    #[test]
    fn test_get_index_wrapped() {
        let grid = Grid::new(5, 5, true, None);

        assert_eq!(grid.get_index(0, 0), Some(0), "Non-Wrapped index (0, 0) should return 0");
        assert_eq!(grid.get_index(4, 4), Some(24), "Non-Wrapped index (0, 0) should return 0");
        assert_eq!(grid.get_index(-1, -1), Some(24), "Index (-1, -1) should wrap and return 24");
        assert_eq!(grid.get_index(10, 10), Some(0), "Index (10, 10) should wrap and return None");
    }

    #[test]
    fn test_get_cell_non_wrapped() {
        let grid = Grid::new(5, 5, false, None);

        assert!(grid.get_cell(2, 3).is_some(), "Cell at valid coordinates (2, 3) should return Some");
        assert!(grid.get_cell(5, 5).is_none(), "Cell at invalid coordinates (5, 5) should return None");
        assert!(grid.get_cell(-2, -3).is_none(), "Cell at invalid coordinates (-2, -3) should return None");
        assert!(grid.get_cell(-2, 3).is_none(), "Cell at invalid coordinates (-2, 3) should return None");
    }

    #[test]
    fn test_get_cell_wrapped() {
        let grid = Grid::new(5, 5, false, None);

        assert!(grid.get_cell(2, 3).is_some(), "Cell at coordinates (2, 3) should return Some");
        assert!(grid.get_cell(5, 5).is_none(), "Cell at coordinates (5, 5) should wrap and return Some");
        assert!(grid.get_cell(-2, -3).is_none(), "Cell at coordinates (-2, -3) should wrap and return Some");
        assert!(grid.get_cell(-2, 3).is_none(), "Cell at coordinates (-2, 3) should wrap and return Some");
    }

    #[test]
    fn test_get_cell_mut() {
        let mut grid = Grid::new(5, 5, true, None);

        let cell = grid.get_cell_mut(2, 3);
        assert!(cell.is_some(), "Mutable cell at valid coordinates should exist");

        if let Some(mut_cell) = cell {
            mut_cell.set_fitness(100);
        }

        // verify modification
        let cell = grid.get_cell(2, 3);
        assert_eq!(cell.unwrap().get_fitness(), 100, "Cell fitness should be modifiable");
    }
}
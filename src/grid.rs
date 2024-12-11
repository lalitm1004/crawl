use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::cell::Cell;

pub struct RngSettings {
    pub seed: u64,
    pub initial_cooperators: f64,
}

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

    fn get_index(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.num_rows || col >= self.num_cols {
            None
        } else {
            Some(row * self.num_cols + col)
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<&Cell> {
        let index = Grid::get_index(&self, row, col);
        if let Some(index) = index {
            self.lattice.get(index)
        } else {
            None
        }
    }

    pub fn get_cell_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
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
    fn test_get_cell_valid_coords() {
        let grid = create_test_grid();
        let cell = grid.get_cell(2, 3);
        assert!(cell.is_some(), "Cell at valid coordinates should exist");
    }

    #[test]
    fn test_get_cell_invalid_coords() {
        let grid = create_test_grid();

        // test coordinates beyond grid size
        assert!(grid.get_cell(5, 0).is_none(), "Row beyond grid size should return None");
        assert!(grid.get_cell(0, 5).is_none(), "Column beyond grid size should return None");
    }

    #[test]
    fn test_get_cell_mut_valid_coords() {
        let mut grid = create_test_grid();

        // test getting a mutable cell at valid coordinates
        let cell = grid.get_cell_mut(2, 3);
        assert!(cell.is_some(), "Mutable cell at valid coordinates should exist");

        if let Some(mut_cell) = cell {
            mut_cell.set_fitness(100);
        }

        // verify modification
        let cell = grid.get_cell(2, 3);
        assert_eq!(cell.unwrap().get_fitness(), 100, "Cell fitness should be modifiable");
    }

    #[test]
    fn test_get_cell_mut_out_of_bounds() {
        let mut grid = create_test_grid();

        assert!(grid.get_cell_mut(5, 0).is_none(), "Row beyond grid size should return None");
        assert!(grid.get_cell_mut(0, 5).is_none(), "Column beyond grid size should return None");
    }

    fn create_test_grid() -> Grid {
        Grid::new(
            5,
            5,
            false,
            Some(RngSettings {
                seed: 100,
                initial_cooperators: 0.5,
            })
        )
    }
}
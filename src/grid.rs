use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::cell::Cell;

pub struct RngSettings {
    pub seed: u64,
    pub initial_cooperators: f64,
}

pub struct Grid {
    pub num_rows: i32,
    pub num_cols: i32,
    pub wrapped: bool,
    pub rng_settings: Option<RngSettings>,
    pub lattice: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(
        num_rows: i32,
        num_cols: i32,
        wrapped: bool,
        rng_settings: Option<RngSettings>
    ) -> Self {
        let lattice: Vec<Vec<Cell>> = match &rng_settings {
            Some(rng_settings) => {
                let mut rng = StdRng::seed_from_u64(rng_settings.seed);
                let threshold = rng_settings.initial_cooperators;
                (0..num_rows)
                    .map(|_| {
                        (0..num_cols)
                            .map(|_| Cell::new(rng.gen::<f64>() < threshold))
                            .collect()
                    })
                    .collect()
            },
            None => {
                println!("No rng settings detected, defaulting to lone defector");
                (0..num_rows)
                    .map(|row| {
                        (0..num_cols)
                            .map(|col| {
                                let is_center_row = row == num_rows / 2;
                                let is_center_col = col == num_cols / 2;

                                Cell::new(!(is_center_row && is_center_col))
                            })
                            .collect()
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

    pub fn get_cell(&self, row: i32, col: i32) -> Option<&Cell> {
        if row < 0 || row >= self.num_rows || col < 0 || col >= self.num_cols {
            return  None;
        }
        self.lattice.get(row as usize)?.get(col as usize)
    }

    pub fn get_cell_mut(&mut self, row: i32, col: i32) -> Option<&mut Cell> {
        if row < 0 || row >= self.num_rows || col < 0 || col >= self.num_cols {
            return  None;
        }
        self.lattice.get_mut(row as usize)?.get_mut(col as usize)
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
            vec![Cell::new(true), Cell::new(false), Cell::new(false), Cell::new(false), Cell::new(true)],
            vec![Cell::new(true), Cell::new(false), Cell::new(false), Cell::new(true), Cell::new(true)],
            vec![Cell::new(false), Cell::new(false), Cell::new(true), Cell::new(true), Cell::new(true)],
            vec![Cell::new(true), Cell::new(true), Cell::new(false), Cell::new(true), Cell::new(false)],
            vec![Cell::new(true), Cell::new(false), Cell::new(true), Cell::new(true), Cell::new(true)],
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
            vec![Cell::new(true), Cell::new(true), Cell::new(true), Cell::new(true), Cell::new(true)],
            vec![Cell::new(true), Cell::new(true), Cell::new(true), Cell::new(true), Cell::new(true)],
            vec![Cell::new(true), Cell::new(true), Cell::new(false), Cell::new(true), Cell::new(true)],
            vec![Cell::new(true), Cell::new(true), Cell::new(true), Cell::new(true), Cell::new(true)],
            vec![Cell::new(true), Cell::new(true), Cell::new(true), Cell::new(true), Cell::new(true)],
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

        // test negative coordinates
        assert!(grid.get_cell(-1, 0).is_none(), "Negative row should return None");
        assert!(grid.get_cell(0, -1).is_none(), "Negative column should return None");

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

        // test negative coordinates
        assert!(grid.get_cell_mut(-1, 0).is_none(), "Negative row should return None");
        assert!(grid.get_cell_mut(0, -1).is_none(), "Negative column should return None");

        // test coordinates beyond grid size
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
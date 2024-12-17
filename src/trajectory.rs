use std::time::{SystemTime, UNIX_EPOCH};

use crate::grid::Grid;
use crate::payoff::PayoffMatrix;
use crate::neighbourhood::Neighbourhood;

#[derive(Debug)]
pub struct Trajectory {
    pub name: String,
    pub id: String,
    pub max_iterations: i32,
    pub curr_iteration: i32,
    pub grid: Grid,
    pub neighbourhood: Neighbourhood,
    pub payoff_matrix: PayoffMatrix,
}

impl Trajectory {
    pub fn new(
        name: String,
        max_iterations: i32,
        grid: Grid,
        neighbourhood: Neighbourhood,
        payoff_matrix: PayoffMatrix
    ) -> Self {
        Self {
            name,
            id: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
            max_iterations,
            curr_iteration: 0,
            grid,
            neighbourhood,
            payoff_matrix,
        }
    }

    fn update_fitness(&mut self) {
        let num_rows = self.grid.num_rows;
        let num_cols = self.grid.num_cols;
        
        let mut fitness = vec![vec![0; num_cols]; num_rows];

        for i in 0..num_rows {
            for j in 0..num_cols {
                let row = i as i32;
                let col = j as i32;

                let curr_cell = self.grid.get_cell(row, col).unwrap();
                let mut total_payoff = 0;

                for (di, dj) in self.neighbourhood.get_neighbourhood() {
                    let neighbour_cell = self.grid.get_cell(row + di, col + dj).unwrap();
                    total_payoff += self.payoff_matrix.get_payoff(&curr_cell, &neighbour_cell);
                }

                fitness[i][j] = total_payoff;
            }
        }

        for i in 0..num_rows {
            for j in 0..num_cols {
                let curr_cell = self.grid.get_cell_mut(i as i32, j as i32).unwrap();
                curr_cell.set_fitness(fitness[i][j]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        grid::Grid,
        neighbourhood::Neighbourhood,
        payoff::PayoffMatrix
    };
    use super::Trajectory;

    #[test]
    fn test_new() {
        let trajectory = Trajectory::new(
            get_name(),
            1000,
            get_grid(),
            get_neighbourhood(),
            get_payoff_matrix(),
        );

        assert_eq!(trajectory.name, get_name());
        assert_eq!(trajectory.max_iterations, 1000);
        assert_eq!(trajectory.curr_iteration, 0);
        assert_eq!(trajectory.grid, get_grid());
        assert_eq!(trajectory.neighbourhood, get_neighbourhood());
        assert_eq!(trajectory.payoff_matrix, get_payoff_matrix());
    }

    #[test]
    fn test_update_fitness() {
        let mut trajectory = Trajectory::new(
            String::from("Test"),
            100,
            get_grid(),
            get_neighbourhood(),
            get_payoff_matrix(),
        );

        trajectory.update_fitness();
        assert_eq!(trajectory.grid.get_cell(2, 2).unwrap().get_fitness(), 400);
        assert_eq!(trajectory.grid.get_cell(2, 1).unwrap().get_fitness(), 100);
        assert_eq!(trajectory.grid.get_cell(2, 0).unwrap().get_fitness(), 200);
    }

    fn get_name() -> String {
        String::from("test")
    }

    fn get_grid() -> Grid {
        Grid::new(5, 5, true, None)
    }

    fn get_neighbourhood() -> Neighbourhood {
        Neighbourhood::von_neumann()
    }

    fn get_payoff_matrix() -> PayoffMatrix {
        PayoffMatrix::new(50, -50, 100, 0)
    }
}
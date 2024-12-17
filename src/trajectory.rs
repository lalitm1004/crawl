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
        let mut fitness = vec![vec![0; self.grid.num_cols as usize]; self.grid.num_rows as usize];

        for (row_idx, row) in fitness.iter_mut().enumerate() {
            for (col_idx, fitness_val) in row.iter_mut().enumerate() {
                let row = row_idx as i32;
                let col = col_idx as i32;

                let curr_cell = self.grid.get_cell(row, col).unwrap();

                let total_payoff= self.neighbourhood.get_neighbourhood()
                    .iter()
                    .map(|&(di, dj)| {
                        let neighbour_cell = self.grid.get_cell(row + di, col + dj).unwrap();
                        self.payoff_matrix.get_payoff(&curr_cell, &neighbour_cell)
                    })
                    .sum();

                *fitness_val = total_payoff;
            }
        }

        for (row_idx, row) in fitness.iter().enumerate() {
            for (col_idx, &fitness_val) in row.iter().enumerate() {
                let curr_cell = self.grid.get_cell_mut(row_idx as i32, col_idx as i32).unwrap();
                curr_cell.set_fitness(fitness_val);
            }
        }
    }

    fn update_strategy(&mut self) {
        for i in 0..self.grid.num_rows {
            for j in 0..self.grid.num_cols {
                let row = i as i32;
                let col = j as i32;

                let curr_cell = self.grid.get_cell(row, col).unwrap();

                let strategy_fitness_map: Vec<(bool, i32)> = self.neighbourhood.get_neighbourhood()
                    .iter()
                    .chain(std::iter::once(&(0, 0))) // include current cell
                    .map(|&(di, dj)| {
                        let cell = self.grid.get_cell(row + di, col + dj).unwrap();
                        (cell.is_cooperator(), cell.get_fitness())
                    })
                    .collect();

                let max_fitness = strategy_fitness_map
                    .iter()
                    .map(|&(_, fitness)| fitness)
                    .max()
                    .unwrap_or(0);

                let maintain_strategy = strategy_fitness_map
                    .iter()
                    .filter(|&&(_, fitness)| fitness == max_fitness)
                    .any(|&(is_neighbour_cooperator, _)| {
                        curr_cell.is_cooperator() == is_neighbour_cooperator
                    });

                let to_cooperator = if maintain_strategy {
                    curr_cell.is_cooperator()
                } else {
                    !curr_cell.is_cooperator()
                };

                self.grid.get_cell_mut(row, col)
                    .unwrap()
                    .update_strategy(to_cooperator);
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
            get_name(),
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

    #[test]
    fn test_update_strategy() {
        let mut trajectory = Trajectory::new(
            get_name(),
            100,
            get_grid(),
            get_neighbourhood(),
            get_payoff_matrix(),
        );

        trajectory.update_fitness();
        trajectory.update_strategy();
        assert_eq!(trajectory.grid.get_cell(2, 2).unwrap().is_cooperator(), false);
        assert_eq!(trajectory.grid.get_cell(2, 1).unwrap().is_cooperator(), false);
        assert_eq!(trajectory.grid.get_cell(2, 0).unwrap().is_cooperator(), true);
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
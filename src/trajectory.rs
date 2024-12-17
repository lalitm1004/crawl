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
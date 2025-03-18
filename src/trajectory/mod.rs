use std::time::{SystemTime, UNIX_EPOCH};

use crate::{grid::Grid, neighbourhood::Neighbourhood, payoff::Payoff};

#[derive(Debug)]
pub struct Trajectory {
    id: String,
    pub name: String,
    pub max_iterations: usize,
    curr_iteration: usize,
    grid: Grid,
    neighbourhood: Neighbourhood,
    payoff: Payoff,

    history: Vec<u64>,

}

impl Trajectory {
    pub fn new(
        name: String,
        max_iterations: usize,
        grid: Grid,
        neighbourhood: Neighbourhood,
        payoff: Payoff,
    ) -> Self {
        Self {
            id: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();
            name,
            max_iterations,
            curr_iteration: 0,
            grid,
            neighbourhood,
            payoff
        }
    }
}
use crate::cell::Cell;

mod matrix;
use matrix::PayoffMatrix;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Payoff {
    pub matrix: PayoffMatrix,
    // add spatial payoff
}

impl Payoff {
    pub fn new(matrix: PayoffMatrix) -> Self {
        Self { matrix }
    }

    pub fn get_payoff(
        &self,
        cell_1: &Cell,
        cell_2: &Cell,
        _coordinates: Option<(i32, i32)>,
    ) -> f32 {
        let matrix_payoff = self.matrix.get_payoff(cell_1, cell_2);

        // add spatial payoff

        matrix_payoff
    }
}

#[cfg(test)]
mod tests;

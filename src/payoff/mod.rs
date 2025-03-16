mod matrix;

use matrix::PayoffMatrix;

pub struct Payoff {
    pub matrix: PayoffMatrix,
    // add spatial payoff
}

impl Payoff {
    pub fn new(matrix: PayoffMatrix) -> Self {
        Self { matrix }
    }
}

#[cfg(test)]
mod tests;

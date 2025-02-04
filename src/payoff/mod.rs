use matrix::PayoffMatrix;
use spatial::SpatialPayoff;

pub mod matrix;
pub mod spatial;

#[derive(Debug)]
pub struct Payoff<T: SpatialPayoff> {
    pub matrix: PayoffMatrix,
    pub spatial: Option<T>
}

impl<T: SpatialPayoff> Payoff<T> {
    pub fn new(matrix: PayoffMatrix, spatial: Option<T>) -> Self {
        Payoff { matrix, spatial }
    }
}

use serde::{Deserialize, Serialize};

use crate::cell::Cell;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PayoffMatrix {
    pub c_c: f32,
    pub c_d: f32,
    pub d_d: f32,
    pub d_c: f32,
}

impl PayoffMatrix {
    pub fn new(c_c: f32, c_d: f32, d_d: f32, d_c: f32) -> Self {
        Self { c_c, c_d, d_d, d_c }
    }

    pub fn get_payoff(&self, cell_1: &Cell, cell_2: &Cell) -> f32 {
        match (cell_1.is_cooperator(), cell_2.is_cooperator()) {
            (true, true) => self.c_c,
            (true, false) => self.c_d,
            (false, false) => self.d_d,
            (false, true) => self.d_c,
        }
    }
}

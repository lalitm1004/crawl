use serde::{Deserialize, Serialize};

use crate::cell::Cell;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct PayoffMatrix {
    c_c: i32,
    c_d: i32,
    d_c: i32,
    d_d: i32,
}

impl PayoffMatrix {
    pub fn new(c_c: i32, c_d: i32, d_c: i32, d_d: i32,) -> Self {
        Self { c_c, c_d, d_c, d_d }
    }

    pub fn get_payoff(&self, cell1: &Cell, cell2: &Cell) -> i32 {
        match cell1.is_cooperator() {
            true => match cell2.is_cooperator() {
                true => self.c_c,
                false => self.c_d,
            }
            false => match cell2.is_cooperator() {
                true => self.d_c,
                false => self.d_d,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cell::Cell;
    use super::PayoffMatrix;

    #[test]
    fn test_get_payoff() {
        let cooperator = Cell::new(true);
        let defector = Cell::new(false);

        let payoff_mat = PayoffMatrix::new(50, -50, 100, 0);
        assert_eq!(payoff_mat.get_payoff(&cooperator, &cooperator), 50, "Invalid payoff");
        assert_eq!(payoff_mat.get_payoff(&cooperator, &defector), -50, "Invalid payoff");
        assert_eq!(payoff_mat.get_payoff(&defector, &cooperator), 100, "Invalid payoff");
        assert_eq!(payoff_mat.get_payoff(&defector, &defector), 0, "Invalid payoff");
    }
}
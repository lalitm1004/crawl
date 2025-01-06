use crate::cell::Cell;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PayoffMatrix {
    c_c: i32,
    c_d: i32,
    d_c: i32,
    d_d: i32,
}

impl PayoffMatrix {
    pub fn new(c_c: i32, c_d: i32, d_c: i32, d_d: i32) -> Self {
        PayoffMatrix { c_c, c_d, d_c, d_d }
    }

    pub fn get_payoff(&self, cell1: &Cell, cell2: &Cell) -> i32 {
        match (cell1.is_cooperator(), cell2.is_cooperator()) {
            (true, true) => self.c_c,
            (true, false) => self.c_d,
            (false, true) => self.d_c,
            (false, false) => self.d_d,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cell, PayoffMatrix};

    #[test]
    fn test_new() {
        let matrix = PayoffMatrix::new(3, 0, 5, 1);
        assert_eq!(matrix.c_c, 3);
        assert_eq!(matrix.c_d, 0);
        assert_eq!(matrix.d_c, 5);
        assert_eq!(matrix.d_d, 1);
    }

    #[test]
    fn test_get_payoff() {
        let matrix = PayoffMatrix::new(3, 0, 5, 1);
        
        let cooperator = Cell::new(true);
        let defector = Cell::new(false);

        assert_eq!(matrix.get_payoff(&cooperator, &cooperator), 3);
        assert_eq!(matrix.get_payoff(&cooperator, &defector), 0);
        assert_eq!(matrix.get_payoff(&defector, &cooperator), 5);
        assert_eq!(matrix.get_payoff(&defector, &defector), 1);
    }
}

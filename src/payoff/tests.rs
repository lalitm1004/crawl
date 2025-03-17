use crate::cell::Cell;

use super::{Payoff, PayoffMatrix};

#[test]
fn test_matrix_new() {
    let matrix = PayoffMatrix::new(3.0, 0.0, 5.0, 1.0);
    assert_eq!(matrix.c_c, 3.0);
    assert_eq!(matrix.c_d, 0.0);
    assert_eq!(matrix.d_d, 5.0);
    assert_eq!(matrix.d_c, 1.0);
}

#[test]
fn test_matrix_get_payoff() {
    let matrix = PayoffMatrix::new(3.0, 0.0, 5.0, 1.0);

    let cooperator = Cell::new(true);
    let defector = Cell::new(false);

    assert_eq!(matrix.get_payoff(&cooperator, &cooperator), 3.0);
    assert_eq!(matrix.get_payoff(&cooperator, &defector), 0.0);
    assert_eq!(matrix.get_payoff(&defector, &defector), 5.0);
    assert_eq!(matrix.get_payoff(&defector, &cooperator), 1.0);
}

#[test]
fn test_payoff_new() {
    let payoff = Payoff::new(PayoffMatrix::new(3.0, 0.0, 5.0, 1.0));

    let cooperator = Cell::new(true);
    let defector = Cell::new(false);

    assert_eq!(payoff.get_payoff(&cooperator, &cooperator, None), 3.0);
    assert_eq!(payoff.get_payoff(&cooperator, &defector, None), 0.0);
    assert_eq!(payoff.get_payoff(&defector, &defector, None), 5.0);
    assert_eq!(payoff.get_payoff(&defector, &cooperator, None), 1.0);
}

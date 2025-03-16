use super::*;

#[test]
fn test_cell_new() {
    assert_eq!(Cell::new(true), Cell::CC(0.0));
    assert_eq!(Cell::new(false), Cell::DD(0.0))
}

#[test]
fn test_cell_is_cooperator() {
    assert!(Cell::CC(0.0).is_cooperator());
    assert!(Cell::DC(0.0).is_cooperator());
    assert!(!Cell::CD(0.0).is_cooperator());
    assert!(!Cell::DD(0.0).is_cooperator());
}

#[test]
fn test_cell_get_fitness() {
    assert_eq!(Cell::CC(5.0).get_fitness(), 5.0);
    assert_eq!(Cell::CD(3.0).get_fitness(), 3.0);
    assert_eq!(Cell::DD(1.0).get_fitness(), 1.0);
    assert_eq!(Cell::DC(7.0).get_fitness(), 7.0);
}

#[test]
fn test_cell_set_fitness() {
    let mut cell_cc = Cell::CC(0.0);
    let mut cell_cd = Cell::CD(1.0);
    let mut cell_dd = Cell::DD(2.0);
    let mut cell_dc = Cell::DC(3.0);

    cell_cc.set_fitness(10.0);
    cell_cd.set_fitness(20.0);
    cell_dd.set_fitness(30.0);
    cell_dc.set_fitness(40.0);

    assert_eq!(cell_cc.get_fitness(), 10.0);
    assert_eq!(cell_cd.get_fitness(), 20.0);
    assert_eq!(cell_dd.get_fitness(), 30.0);
    assert_eq!(cell_dc.get_fitness(), 40.0);
}

#[test]
fn test_cell_update_strategy() {
    // test transitions to cooperator
    let mut cell = Cell::DD(5.0);
    cell.update_strategy(true);
    assert_eq!(cell, Cell::DC(5.0));

    cell = Cell::CD(5.0);
    cell.update_strategy(true);
    assert_eq!(cell, Cell::DC(5.0));

    cell = Cell::CC(5.0);
    cell.update_strategy(true);
    assert_eq!(cell, Cell::CC(5.0));

    cell = Cell::DC(5.0);
    cell.update_strategy(true);
    assert_eq!(cell, Cell::CC(5.0));

    // test transitions to defector
    cell = Cell::CC(5.0);
    cell.update_strategy(false);
    assert_eq!(cell, Cell::CD(5.0));

    cell = Cell::DC(5.0);
    cell.update_strategy(false);
    assert_eq!(cell, Cell::CD(5.0));

    cell = Cell::DD(5.0);
    cell.update_strategy(false);
    assert_eq!(cell, Cell::DD(5.0));

    cell = Cell::CD(5.0);
    cell.update_strategy(false);
    assert_eq!(cell, Cell::DD(5.0));
}

#[test]
fn test_cell_fitness_preservation() {
    let mut cell = Cell::CC(42.0);
    cell.update_strategy(false);
    assert_eq!(cell.get_fitness(), 42.0);
    cell.update_strategy(true);
    assert_eq!(cell.get_fitness(), 42.0);
}

#[test]
fn test_cell_multiple_transitions() {
    let mut cell = Cell::CC(10.0);

    cell.update_strategy(false);
    assert_eq!(cell, Cell::CD(10.0));

    cell.update_strategy(false);
    assert_eq!(cell, Cell::DD(10.0));

    cell.update_strategy(true);
    assert_eq!(cell, Cell::DC(10.0));

    cell.update_strategy(true);
    assert_eq!(cell, Cell::CC(10.0));
}

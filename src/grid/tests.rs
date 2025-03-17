use super::{Grid, rng::RngSettings};

#[test]
fn test_get_index_non_wrapped() {
    let grid = Grid::new((5, 5), false, None);

    assert_eq!(
        grid.get_index(0, 0),
        Some(0),
        "Non-Wrapped index (0, 0) should return 0"
    );
    assert_eq!(
        grid.get_index(4, 4),
        Some(24),
        "Non-Wrapped index (4, 4) should return 24"
    );
    assert_eq!(
        grid.get_index(-1, -1),
        None,
        "Index out of bounds should return None"
    );
    assert_eq!(
        grid.get_index(10, 10),
        None,
        "Index out of bounds should return None"
    );
}

#[test]
fn test_get_index_wrapped() {
    let grid = Grid::new((5, 5), true, None);

    assert_eq!(
        grid.get_index(0, 0),
        Some(0),
        "Non-Wrapped index (0, 0) should return 0"
    );
    assert_eq!(
        grid.get_index(4, 4),
        Some(24),
        "Non-Wrapped index (0, 0) should return 0"
    );
    assert_eq!(
        grid.get_index(-1, -1),
        Some(24),
        "Index (-1, -1) should wrap and return 24"
    );
    assert_eq!(
        grid.get_index(10, 10),
        Some(0),
        "Index (10, 10) should wrap and return 0"
    );
}

#[test]
fn test_get_cell_non_wrapped() {
    let grid = Grid::new((5, 5), false, None);

    assert!(
        grid.get_cell(2, 3).is_some(),
        "Cell at valid coordinates (2, 3) should return Some"
    );
    assert!(
        grid.get_cell(5, 5).is_none(),
        "Cell at invalid coordinates (5, 5) should return None"
    );
    assert!(
        grid.get_cell(-2, -3).is_none(),
        "Cell at invalid coordinates (-2, -3) should return None"
    );
    assert!(
        grid.get_cell(-2, 3).is_none(),
        "Cell at invalid coordinates (-2, 3) should return None"
    );
}

#[test]
fn test_get_cell_wrapped() {
    let grid = Grid::new((5, 5), false, None);

    assert!(
        grid.get_cell(2, 3).is_some(),
        "Cell at coordinates (2, 3) should return Some"
    );
    assert!(
        grid.get_cell(5, 5).is_none(),
        "Cell at coordinates (5, 5) should wrap and return Some"
    );
    assert!(
        grid.get_cell(-2, -3).is_none(),
        "Cell at coordinates (-2, -3) should wrap and return Some"
    );
    assert!(
        grid.get_cell(-2, 3).is_none(),
        "Cell at coordinates (-2, 3) should wrap and return Some"
    );
}

#[test]
fn test_get_cell_mut() {
    let mut grid = Grid::new((5, 5), true, None);

    let cell = grid.get_cell_mut(2, 3);
    assert!(
        cell.is_some(),
        "Mutable cell at valid coordinates should exist"
    );

    if let Some(mut_cell) = cell {
        mut_cell.set_fitness(100.0);
    }

    // verify modification
    let cell = grid.get_cell(2, 3);
    assert_eq!(
        cell.unwrap().get_fitness(),
        100.0,
        "Cell fitness should be modifiable"
    );
}

#[test]
fn test_hash_consistency() -> Result<(), String> {
    let grid = Grid::new((100, 100), true, Some(RngSettings::new(None, 0.5)?));

    let hash_1 = grid.get_lattice_hash();
    let hash_2 = grid.get_lattice_hash();

    assert_eq!(hash_1, hash_2);
    Ok(())
}

#[test]
fn test_hash_on_change() -> Result<(), String> {
    let mut grid = Grid::new((100, 100), true, Some(RngSettings::new(None, 0.5)?));

    let hash_1 = grid.get_lattice_hash();

    let cell = grid.get_cell_mut(0, 0).unwrap();
    cell.update_strategy(!cell.is_cooperator());

    let hash_2 = grid.get_lattice_hash();

    assert_ne!(hash_1, hash_2);
    Ok(())
}

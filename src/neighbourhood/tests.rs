use super::*;

#[test]
fn test_direction_to_offset() {
    assert_eq!(*Direction::Up.to_offset(), (0, -1));
    assert_eq!(*Direction::TopRight.to_offset(), (1, -1));
    assert_eq!(*Direction::Right.to_offset(), (1, 0));
    assert_eq!(*Direction::BottomRight.to_offset(), (1, 1));
    assert_eq!(*Direction::Down.to_offset(), (0, 1));
    assert_eq!(*Direction::BottomLeft.to_offset(), (-1, 1));
    assert_eq!(*Direction::Left.to_offset(), (-1, 0));
    assert_eq!(*Direction::TopLeft.to_offset(), (-1, -1));
}

#[test]
fn test_moore_neighbourhood() {
    let neighbourhood = Neighbourhood::moore();
    let expected_directions = vec![
        Direction::Up,
        Direction::TopRight,
        Direction::Right,
        Direction::BottomRight,
        Direction::Down,
        Direction::BottomLeft,
        Direction::Left,
        Direction::TopLeft,
    ];

    assert_eq!(neighbourhood.get_directions(), &expected_directions);

    let offsets: Vec<(i32, i32)> = neighbourhood.offsets_iter().cloned().collect();
    let expected_offsets: Vec<(i32, i32)> =
        expected_directions.iter().map(|d| *d.to_offset()).collect();
    assert_eq!(offsets, expected_offsets);
}

#[test]
fn test_von_neumann_neighbourhood() {
    let neighbourhood = Neighbourhood::von_neumann();
    let expected_directions = vec![
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    assert_eq!(neighbourhood.get_directions(), &expected_directions);

    let offsets: Vec<(i32, i32)> = neighbourhood.offsets_iter().cloned().collect();
    let expected_offsets: Vec<(i32, i32)> =
        expected_directions.iter().map(|d| *d.to_offset()).collect();
    assert_eq!(offsets, expected_offsets);
}

#[test]
fn test_custom_neighbourhood() {
    let custom_directions = vec![Direction::Up, Direction::Left, Direction::Down];
    let neighbourhood = Neighbourhood::custom(custom_directions.clone());

    assert_eq!(neighbourhood.get_directions(), &custom_directions);

    let offsets: Vec<(i32, i32)> = neighbourhood.offsets_iter().cloned().collect();
    let expected_offsets: Vec<(i32, i32)> =
        custom_directions.iter().map(|d| *d.to_offset()).collect();
    assert_eq!(offsets, expected_offsets);
}

#[test]
fn test_offsets_iter_repeated_calls() {
    let neighbourhood = Neighbourhood::moore();
    let offsets_first: Vec<(i32, i32)> = neighbourhood.offsets_iter().cloned().collect();
    let offsets_second: Vec<(i32, i32)> = neighbourhood.offsets_iter().cloned().collect();
    assert_eq!(offsets_first, offsets_second,);
}

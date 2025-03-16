#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    TopRight,
    Right,
    BottomRight,
    Down,
    BottomLeft,
    Left,
    TopLeft,
}

const OFFSETS: &[(i32, i32)] = &[
    (0, -1),
    (1, -1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, -1),
];

#[derive(Debug)]
pub struct Neighbourhood {
    neighbours: Vec<Direction>,
}

impl Direction {
    pub const fn to_offset(&self) -> &(i32, i32) {
        match self {
            Direction::Up => &OFFSETS[0],
            Direction::TopRight => &OFFSETS[1],
            Direction::Right => &OFFSETS[2],
            Direction::BottomRight => &OFFSETS[3],
            Direction::Down => &OFFSETS[4],
            Direction::BottomLeft => &OFFSETS[5],
            Direction::Left => &OFFSETS[6],
            Direction::TopLeft => &OFFSETS[7],
        }
    }
}

impl Neighbourhood {
    pub fn custom(neighbours: Vec<Direction>) -> Self {
        Self { neighbours }
    }

    pub fn moore() -> Self {
        Self {
            neighbours: vec![
                Direction::Up,
                Direction::TopRight,
                Direction::Right,
                Direction::BottomRight,
                Direction::Down,
                Direction::BottomLeft,
                Direction::Left,
                Direction::TopLeft,
            ],
        }
    }

    pub fn von_neumann() -> Self {
        Self {
            neighbours: vec![
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ],
        }
    }

    pub fn get_directions(&self) -> &Vec<Direction> {
        &self.neighbours
    }

    pub fn offsets_iter(&self) -> impl Iterator<Item = &(i32, i32)> {
        self.neighbours.iter().map(|d| d.to_offset())
    }
}

#[cfg(test)]
mod tests;

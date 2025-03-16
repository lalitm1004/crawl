#[derive(Debug, Clone, Copy, PartialEq)]
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
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

#[derive(Debug)]
pub struct Neighbourhood {
    neighbours: Vec<Direction>,
}

impl Direction {
    #[inline]
    pub const fn to_offset(&self) -> &'static (i32, i32) {
        &OFFSETS[*self as usize]
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

    #[inline]
    pub fn get_directions(&self) -> &[Direction] {
        &self.neighbours
    }

    #[inline]
    pub fn offsets_iter(&self) -> impl Iterator<Item = &(i32, i32)> {
        self.neighbours.iter().map(|d| d.to_offset())
    }
}

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Neighbourhood {
    neighbors: Vec<Direction>,
}

impl Neighbourhood {
    pub fn custom(directions: Vec<Direction>) -> Self {
        Neighbourhood {
            neighbors: directions
        }
    }

    // convention for listing neighbours is to start with Up and move clockwise
    pub fn moore() -> Self {
        Neighbourhood {
            neighbors: vec![
                Direction::Up,
                Direction::TopRight,
                Direction::Right,
                Direction::BottomRight,
                Direction::Down,
                Direction::BottomLeft,
                Direction::Left,
                Direction::TopLeft,
            ]
        }
    }

    pub fn von_neumann() -> Self {
        Neighbourhood {
            neighbors: vec![
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ],
        }
    }

    pub fn get_directions(&self) -> Vec<Direction> {
        self.neighbors.clone()
    }

    pub fn get_neighbourhood(&self) -> Vec<(i32, i32)> {
        self.neighbors
            .iter()
            .map(Self::direction_to_offset)
            .collect()
    }

    fn direction_to_offset(direction: &Direction) -> (i32 , i32) {
        match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::TopRight => (1, -1),
            Direction::TopLeft => (-1, -1),
            Direction::BottomRight => (1, 1),
            Direction::BottomLeft => (-1, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use  super::{Direction, Neighbourhood};

    #[test]
    fn test_moore_neighbourhood() {
        let moore = Neighbourhood::moore();
        let expected = vec![
            (0, -1), (1, -1), (1, 0), (1, 1),
            (0, 1), (-1, 1), (-1, 0), (-1, -1),
        ];
        assert_eq!(moore.get_neighbourhood(), expected, "Invalid Moore neighbourhood");
    }

    #[test]
    fn test_von_neumann_neighbourhood() {
        let von_neumann = Neighbourhood::von_neumann();
        let expected = vec![
            (0, -1), (1, 0),
            (0, 1), (-1, 0)
        ];
        assert_eq!(von_neumann.get_neighbourhood(), expected, "Invalid VonNeumann neighbourhood");
    }

    #[test]
    fn test_custom_neighbourhood() {
        let custom = Neighbourhood::custom(vec![
            Direction::Up,
            Direction::Down,
        ]);
        let expected = vec![(0, -1), (0, 1)];
        assert_eq!(custom.get_neighbourhood(), expected);

        // empty neighbourhood
        let custom = Neighbourhood::custom(vec![]);
        let expected: Vec<(i32, i32)> = vec![];
        assert_eq!(custom.get_neighbourhood(), expected);
    }
}
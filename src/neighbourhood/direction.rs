#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
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

impl Direction {
    #[inline]
    pub const fn to_offset(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::TopRight => (1, -1),
            Direction::Right => (1, 0),
            Direction::BottomRight => (1, 1),
            Direction::Down => (0, 1),
            Direction::BottomLeft => (-1, 1),
            Direction::Left => (-1, 0),
            Direction::TopLeft => (-1, -1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Direction;

    #[test]
    fn test_direction_to_offset() {
        assert_eq!(Direction::Up.to_offset(), (0, -1));
        assert_eq!(Direction::TopRight.to_offset(), (1, -1));
        assert_eq!(Direction::Right.to_offset(), (1, 0));
        assert_eq!(Direction::BottomRight.to_offset(), (1, 1));
        assert_eq!(Direction::Down.to_offset(), (0, 1));
        assert_eq!(Direction::BottomLeft.to_offset(), (-1, 1));
        assert_eq!(Direction::Left.to_offset(), (-1, 0));
        assert_eq!(Direction::TopLeft.to_offset(), (-1, -1));
    }
}

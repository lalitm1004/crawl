pub mod direction;
use direction::Direction;

pub struct Neighbourhood {
    neighbours: Vec<Direction>
}

impl Neighbourhood {
    #[inline]
    pub fn custom(directions: Vec<Direction>) -> Self {
        Neighbourhood {
            neighbours: directions
        }
    }

    pub fn moore() -> Self {
        Neighbourhood {
            neighbours: vec![
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
            neighbours: vec![
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ]
        }
    }

    #[inline]
    pub fn get_directions(&self) -> &Vec<Direction> {
        &self.neighbours
    }

    #[inline]
    pub fn get_offsets(&self) -> Vec<(i32, i32)> {
        self.neighbours
            .iter()
            .map(|&direction| direction.to_offset())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, Neighbourhood};

    #[test]
    fn test_neighbourhood_moore() {
        let moore = Neighbourhood::moore();
        assert_eq!(moore.get_directions().len(), 8);
        assert!(moore.get_directions().contains(&Direction::TopRight));
        
        let offsets = moore.get_offsets();
        assert_eq!(offsets.len(), 8);
        assert!(offsets.contains(&(1, -1)));
    }

    #[test]
    fn test_neighbourhood_von_neumann() {
        let von_neumann = Neighbourhood::von_neumann();
        assert_eq!(von_neumann.get_directions().len(), 4);
        assert!(!von_neumann.get_directions().contains(&Direction::TopRight));
        
        let offsets = von_neumann.get_offsets();
        assert_eq!(offsets.len(), 4);
        assert!(!offsets.contains(&(1, -1)));
    }

    #[test]
    fn test_custom_neighbourhood() {
        let custom = Neighbourhood::custom(vec![Direction::Up, Direction::Down]);
        assert_eq!(custom.get_directions().len(), 2);
        assert!(custom.get_directions().contains(&Direction::Up));
        assert!(custom.get_directions().contains(&Direction::Down));
        
        let offsets = custom.get_offsets();
        assert_eq!(offsets.len(), 2);
        assert!(offsets.contains(&(0, -1)));
        assert!(offsets.contains(&(0, 1)));
    }

    #[test]
    fn test_empty_neighbourhood() {
        let custom = Neighbourhood::custom(vec![]);
        assert!(custom.get_directions().is_empty());
        assert!(custom.get_offsets().is_empty());
    }


    #[test]
    fn test_duplicate_directions() {
        let custom = Neighbourhood::custom(vec![Direction::Up, Direction::Up]);
        assert_eq!(custom.get_directions().len(), 2);
    }
}

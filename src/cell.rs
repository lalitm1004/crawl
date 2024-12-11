#[derive(PartialEq, Debug)]
pub enum Cell {
    CC(i32),
    CD(i32),
    DD(i32),
    DC(i32),
}

impl Cell {
    pub fn new(is_cooperator: bool) -> Self {
        match is_cooperator {
            true => Self::CC(0),
            false => Self::DD(0),
        }
    }

    pub fn is_cooperator(&self) -> bool {
        matches!(self, Self::CC(_) | Self::DC(_))
    }

    pub fn get_fitness(&self) -> i32 {
        match self {
            Self::CC(fitness) |
            Self::CD(fitness) |
            Self::DD(fitness) |
            Self::DC(fitness) => *fitness
        }
    }

    pub fn set_fitness(&mut self, new_fitness: i32) {
        *match self {
            Self::CC(fitness) => fitness,
            Self::CD(fitness) => fitness,
            Self::DD(fitness) => fitness,
            Self::DC(fitness) => fitness,
        } = new_fitness;
    }

    pub fn increment_fitness(&mut self, increment: i32) {
        *match self {
            Self::CC(fitness) => fitness,
            Self::CD(fitness) => fitness,
            Self::DD(fitness) => fitness,
            Self::DC(fitness) => fitness,
        } += increment;
    }

    pub fn update_strategy(&mut self, to_cooperator: bool) {
        let fitness = self.get_fitness();
        *self = match to_cooperator {
            true => match self {
                Self::DD(_) | Self::CD(_) => Self::DC(fitness),
                _ => Self::CC(fitness)
            },
            false => match self {
                Self::CC(_) | Self::DC(_) => Self::CD(fitness),
                _ => Self::DD(fitness)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn test_new() {
        let cooperator = Cell::new(true);
        assert!(matches!(cooperator, Cell::CC(0)), "New cooperator should be CC variant with fitness 0");

        let defector = Cell::new(false);
        assert!(matches!(defector, Cell::DD(0)), "New defector should be DD variant with fitness 0");
    }

    #[test]
    fn test_is_cooperator() {
        assert!(Cell::CC(0).is_cooperator(), "CC should be cooperator");
        assert!(Cell::DC(20).is_cooperator(), "DC should be cooperator");
        assert!(!Cell::CD(-10).is_cooperator(), "CD should be defector");
        assert!(!Cell::DD(-0).is_cooperator(), "DD should be defector");
    }

    #[test]
    fn test_get_fitness() {
        assert_eq!(Cell::CC(10).get_fitness(), 10, "Fitness of CC should be 10");
        assert_eq!(Cell::CD(20).get_fitness(), 20, "Fitness of CD should be 20");
        assert_eq!(Cell::DC(-10).get_fitness(), -10, "Fitness of DC should be -10");
        assert_eq!(Cell::DD(-20).get_fitness(), -20, "Fitness of DD should be -20");
    }

    #[test]
    fn test_set_fitness() {
        let mut strategy = Cell::CC(5);

        strategy.set_fitness(20);
        assert_eq!(strategy.get_fitness(), 20, "Fitness should be 20");

        strategy.set_fitness(-2);
        assert_eq!(strategy.get_fitness(), -2, "Fitness should be -2");
    }

    #[test]
    fn test_increment_fitness() {
        let mut strategy = Cell::DC(5);

        strategy.increment_fitness(20);
        assert_eq!(strategy.get_fitness(), 25, "Fitness should be 25");

        strategy.increment_fitness(-25);
        assert_eq!(strategy.get_fitness(), 0, "Fitness should be 0");

        strategy.increment_fitness(-25);
        assert_eq!(strategy.get_fitness(), -25, "Fitness should be -25");
    }

    #[test]
    fn test_update_strategy() {
        let mut cell = Cell::CC(100);

        cell.update_strategy(false);
        assert_eq!(cell, Cell::CD(100));

        cell.update_strategy(false);
        assert_eq!(cell, Cell::DD(100));

        cell.update_strategy(true);
        assert_eq!(cell, Cell::DC(100));

        cell.update_strategy(true);
        assert_eq!(cell, Cell::CC(100));
    }
}
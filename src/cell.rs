#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Strategy {
    Cooperator,
    Defector
}

#[derive(Debug, PartialEq)]
pub struct Cell {
    strategy: Strategy,
    previous_strategy: Strategy,
    fitness: i32,
}

impl Cell {
    pub fn new(is_cooperator: bool) -> Self {
        match is_cooperator {
            true => Self {
                strategy: Strategy::Cooperator,
                previous_strategy: Strategy::Cooperator,
                fitness: 0,
            },
            false => Self {
                strategy: Strategy::Defector,
                previous_strategy: Strategy::Defector,
                fitness: 0,
            }
        }
    }

    pub fn is_cooperator(&self) -> bool {
        self.strategy == Strategy::Cooperator
    }

    pub fn get_fitness(&self) -> i32 {
        self.fitness
    }

    pub fn set_fitness(&mut self, new_fitness: i32) {
        self.fitness = new_fitness;
    }

    pub fn increment_fitness(&mut self, increment: i32) {
        self.fitness += increment;
    }

    pub fn update_strategy(&mut self, to_cooperator: bool) {
        self.previous_strategy = self.strategy;
        self.strategy = match to_cooperator {
            true => Strategy::Cooperator,
            false => Strategy::Defector,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Strategy, Cell};

    #[test]
    fn test_new() {
        let cooperator = Cell::new(true);
        assert_eq!(cooperator.strategy, Strategy::Cooperator);
        assert_eq!(cooperator.previous_strategy, Strategy::Cooperator);
        assert_eq!(cooperator.fitness, 0);

        let defector = Cell::new(false);
        assert_eq!(defector.strategy, Strategy::Defector);
        assert_eq!(defector.previous_strategy, Strategy::Defector);
        assert_eq!(defector.fitness, 0);
    }

    #[test]
    fn test_is_cooperator() {
        let cooperator = Cell::new(true);
        assert!(cooperator.is_cooperator(), "Should be cooperator");

        let defector = Cell::new(false);
        assert!(!defector.is_cooperator(), "Should be defector");
    }

    #[test]
    fn test_get_fitness() {
        let mut cell = Cell::new(true);
        cell.fitness = 10;
        assert_eq!(cell.get_fitness(), 10, "Fitness should be 10");
    }

    #[test]
    fn test_set_fitness() {
        let mut cell = Cell::new(true);
        
        cell.set_fitness(20);
        assert_eq!(cell.get_fitness(), 20, "Fitness should be 20");

        cell.set_fitness(-2);
        assert_eq!(cell.get_fitness(), -2, "Fitness should be -2");
    }

    #[test]
    fn test_increment_fitness() {
        let mut cell = Cell::new(true);

        cell.increment_fitness(20);
        assert_eq!(cell.get_fitness(), 20, "Fitness should be 20");

        cell.increment_fitness(-10);
        assert_eq!(cell.get_fitness(), 10, "Fitness should be 10");

        cell.increment_fitness(-25);
        assert_eq!(cell.get_fitness(), -15, "Fitness should be -15");
    }

    #[test]
    fn test_update_strategy() {
        let mut cell = Cell::new(true);
        
        assert!(cell.is_cooperator());
        assert_eq!(cell.strategy, Strategy::Cooperator);
        assert_eq!(cell.previous_strategy, Strategy::Cooperator);

        cell.update_strategy(false);
        assert!(!cell.is_cooperator());
        assert_eq!(cell.strategy, Strategy::Defector);
        assert_eq!(cell.previous_strategy, Strategy::Cooperator);

        cell.update_strategy(true);
        assert!(cell.is_cooperator());
        assert_eq!(cell.strategy, Strategy::Cooperator);
        assert_eq!(cell.previous_strategy, Strategy::Defector);
    }
}
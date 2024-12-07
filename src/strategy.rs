#[allow(dead_code)]
enum Strategy {
    CC(i32),
    CD(i32),
    DD(i32),
    DC(i32),
}

#[allow(dead_code)]
impl Strategy {
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
}

#[cfg(test)]
mod test {
    use super::Strategy;

    #[test]
    fn test_new() {
        let cooperator = Strategy::new(true);
        assert!(matches!(cooperator, Strategy::CC(0)), "New cooperator should be CC variant with fitness 0");

        let defector = Strategy::new(false);
        assert!(matches!(defector, Strategy::DD(0)), "New defector should be DD variant with fitness 0");
    }

    #[test]
    fn test_is_cooperator() {
        assert!(Strategy::CC(0).is_cooperator(), "CC should be cooperator");
        assert!(Strategy::DC(20).is_cooperator(), "DC should be cooperator");
        assert!(!Strategy::CD(-10).is_cooperator(), "CD should be defector");
        assert!(!Strategy::DD(-0).is_cooperator(), "DD should be defector");
    }

    #[test]
    fn test_get_fitness() {
        assert_eq!(Strategy::CC(10).get_fitness(), 10, "Fitness of CC should be 10");
        assert_eq!(Strategy::CD(20).get_fitness(), 20, "Fitness of CD should be 20");
        assert_eq!(Strategy::DC(-10).get_fitness(), -10, "Fitness of DC should be -10");
        assert_eq!(Strategy::DD(-20).get_fitness(), -20, "Fitness of DD should be -20");
    }

    #[test]
    fn test_set_fitness() {
        let mut strategy = Strategy::CC(5);

        strategy.set_fitness(20);
        assert_eq!(strategy.get_fitness(), 20, "Fitness should be 20");

        strategy.set_fitness(-2);
        assert_eq!(strategy.get_fitness(), -2, "Fitness should be -2");
    }

    #[test]
    fn test_increment_fitness() {
        let mut strategy = Strategy::DC(5);

        strategy.increment_fitness(20);
        assert_eq!(strategy.get_fitness(), 25, "Fitness should be 25");

        strategy.increment_fitness(-25);
        assert_eq!(strategy.get_fitness(), 0, "Fitness should be 0");

        strategy.increment_fitness(-25);
        assert_eq!(strategy.get_fitness(), -25, "Fitness should be -25");
    }
}
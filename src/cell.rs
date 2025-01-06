#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Cell {
    CC(i32),
    CD(i32),
    DD(i32),
    DC(i32),
}

impl Cell {
    pub fn new(is_cooperator: bool) -> Self {
        match is_cooperator {
            true => Cell::CC(0),
            false => Cell::DD(0),
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

    pub fn update_strategy(&mut self, to_cooperator: bool) {
        let fitness = self.get_fitness();
        *self = match (self.is_cooperator(), to_cooperator) {
            (true, true) => Self::CC(fitness),
            (true, false) => Self::CD(fitness),
            (false, true) => Self::DC(fitness),
            (false, false) => Self::DD(fitness)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn test_new() {
        assert_eq!(Cell::new(true), Cell::CC(0));
        assert_eq!(Cell::new(false), Cell::DD(0));
    }

    #[test]
    fn test_is_cooperator() {
        assert!(Cell::CC(0).is_cooperator());
        assert!(Cell::DC(0).is_cooperator());
        assert!(!Cell::CD(0).is_cooperator());
        assert!(!Cell::DD(0).is_cooperator());
    }

    #[test]
    fn test_get_fitness() {
        assert_eq!(Cell::CC(5).get_fitness(), 5);
        assert_eq!(Cell::CD(3).get_fitness(), 3);
        assert_eq!(Cell::DD(1).get_fitness(), 1);
        assert_eq!(Cell::DC(7).get_fitness(), 7);
    }

    #[test]
    fn test_set_fitness() {
        let mut cell_cc = Cell::CC(0);
        let mut cell_cd = Cell::CD(1);
        let mut cell_dd = Cell::DD(2);
        let mut cell_dc = Cell::DC(3);

        cell_cc.set_fitness(10);
        cell_cd.set_fitness(20);
        cell_dd.set_fitness(30);
        cell_dc.set_fitness(40);

        assert_eq!(cell_cc.get_fitness(), 10);
        assert_eq!(cell_cd.get_fitness(), 20);
        assert_eq!(cell_dd.get_fitness(), 30);
        assert_eq!(cell_dc.get_fitness(), 40);
    }

    #[test]
    fn test_update_strategy() {
        // test transitions to cooperator
        let mut cell = Cell::DD(5);
        cell.update_strategy(true);
        assert_eq!(cell, Cell::DC(5));

        cell = Cell::CD(5);
        cell.update_strategy(true);
        assert_eq!(cell, Cell::DC(5));

        cell = Cell::CC(5);
        cell.update_strategy(true);
        assert_eq!(cell, Cell::CC(5));

        cell = Cell::DC(5);
        cell.update_strategy(true);
        assert_eq!(cell, Cell::CC(5));

        // test transitions to defector
        cell = Cell::CC(5);
        cell.update_strategy(false);
        assert_eq!(cell, Cell::CD(5));

        cell = Cell::DC(5);
        cell.update_strategy(false);
        assert_eq!(cell, Cell::CD(5));

        cell = Cell::DD(5);
        cell.update_strategy(false);
        assert_eq!(cell, Cell::DD(5));

        cell = Cell::CD(5);
        cell.update_strategy(false);
        assert_eq!(cell, Cell::DD(5));
    }

    #[test]
    fn test_fitness_preservation() {
        let mut cell = Cell::CC(42);
        cell.update_strategy(false);
        assert_eq!(cell.get_fitness(), 42);
        cell.update_strategy(true);
        assert_eq!(cell.get_fitness(), 42);
    }

    #[test]
    fn test_multiple_transitions() {
        let mut cell = Cell::CC(10);

        cell.update_strategy(false);
        assert_eq!(cell, Cell::CD(10));

        cell.update_strategy(false);
        assert_eq!(cell, Cell::DD(10));

        cell.update_strategy(true);
        assert_eq!(cell, Cell::DC(10));

        cell.update_strategy(true);
        assert_eq!(cell, Cell::CC(10));
    }
}
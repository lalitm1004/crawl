#[derive(Debug)]
pub enum Cell {
    CC(f32),
    CD(f32),
    DD(f32),
    DC(f32),
}

impl Cell {
    pub fn new(is_cooperator: bool) -> Self {
        match is_cooperator {
            true => Cell::CC(0.0),
            false => Cell::DD(0.0),
        }
    }

    pub fn is_cooperator(&self) -> bool {
        matches!(self, Cell::CC(_) | Cell::DC(_))
    }

    pub fn get_fitness(&self) -> f32 {
        *match self {
            Cell::CC(fitness) | Cell::CD(fitness) | Cell::DD(fitness) | Cell::DC(fitness) => {
                fitness
            }
        }
    }

    pub fn set_fitness(&mut self, new_fitness: f32) {
        *match self {
            Cell::CC(fitness) | Cell::CD(fitness) | Cell::DD(fitness) | Cell::DC(fitness) => {
                fitness
            }
        } = new_fitness;
    }

    pub fn update_strategy(&mut self, to_cooperator: bool) -> Self {
        let fitness = self.get_fitness();

        match (self.is_cooperator(), to_cooperator) {
            (true, true) => Cell::CC(fitness),
            (true, false) => Cell::CD(fitness),
            (false, false) => Cell::DD(fitness),
            (false, true) => Cell::DC(fitness),
        }
    }
}

#[cfg(test)]
mod tests;

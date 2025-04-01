use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    grid::{Grid, RngSettings},
    neighbourhood::{Direction, Neighbourhood},
    payoff::Payoff,
};

#[derive(Debug)]
pub struct Trajectory {
    id: String,
    pub name: String,
    pub max_iterations: usize,
    curr_iteration: usize,
    grid: Grid,
    neighbourhood: Neighbourhood,
    payoff: Payoff,

    history: Vec<u64>,
}

impl Trajectory {
    pub fn new(
        name: String,
        max_iterations: usize,
        grid: Grid,
        neighbourhood: Neighbourhood,
        payoff: Payoff,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let trajectory = Self {
            id: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
            name,
            max_iterations,
            curr_iteration: 0,
            grid,
            neighbourhood,
            payoff,
            history: vec![],
        };

        trajectory.initialize_trajectory()?;

        Ok(trajectory)
    }

    fn initialize_trajectory(&self) -> Result<(), Box<dyn std::error::Error>> {
        let base_path = Path::new("trajectories").join(&self.name).join(&self.id);
        std::fs::create_dir_all(&base_path.clone())?;

        let metadata_json = self.serialize_metadata()?;

        let metadata_path = base_path.join("metadata.json");
        let mut metadata_file = File::create(metadata_path)?;
        metadata_file.write_all(metadata_json.as_bytes())?;

        Ok(())
    }

    fn serialize_metadata(&self) -> Result<String, serde_json::Error> {
        #[derive(Serialize)]
        struct TrajectoryMetadata<'a> {
            id: &'a str,
            name: &'a str,
            max_iterations: usize,
            neighbourhood: Vec<Direction>,
            payoff: Payoff,
            grid: GridMetadata,
        }

        #[derive(Serialize)]
        struct GridMetadata {
            dimension: (i32, i32),
            wrapped: bool,
            rng_settings: Option<RngSettings>,
        }

        let metadata = TrajectoryMetadata {
            name: &self.name,
            id: &self.id,
            max_iterations: self.max_iterations,
            neighbourhood: self.neighbourhood.get_directions().into(),
            payoff: self.payoff,
            grid: GridMetadata {
                dimension: self.grid.dimension,
                wrapped: self.grid.wrapped,
                rng_settings: self.grid.rng_settings.clone(),
            },
        };

        serde_json::to_string_pretty(&metadata)
    }
}

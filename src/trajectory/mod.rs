use std::{
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH}
};
use serde::{Serialize, Deserialize};

use crate::{
    grid::{Grid, RngSettings},
    neighbourhood::{
        direction::Direction,
        Neighbourhood
    },
    payoff::{
        matrix::PayoffMatrix,
        spatial::SpatialPayoff,
        Payoff
    }
};

pub struct Trajectory<T: SpatialPayoff> {
    pub name: String,
    pub id: String,
    pub max_iterations: usize,
    pub curr_iteration: usize,
    pub grid: Grid,
    pub neighbourhood: Neighbourhood,
    pub payoff: Payoff<T>
}

impl<T: SpatialPayoff> Trajectory<T> {
    pub fn new(
        name: String,
        max_iterations: usize,
        grid: Grid,
        neighbourhood: Neighbourhood,
        payoff: Payoff<T>
    ) -> Self {
        Trajectory {
            name,
            id: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
            max_iterations,
            curr_iteration: 0,
            grid,
            neighbourhood,
            payoff,
        }
    }

    fn initialize(&self) {
        let base_path = Path::new("trajectories").join(&self.name).join(&self.id);
        std::fs::create_dir_all(base_path.clone()).expect("error creating folders");

        let metadata = self.get_metadata().expect("error serializing metadata");

        let metadata_path = base_path.join("metadata.json");
        let mut metadata_file = std::fs::File::create(metadata_path).expect("error creating metadata.json");
        metadata_file.write_all(metadata.as_bytes()).expect("error writing to metadata.json");
    }

    fn get_metadata(&self) -> Result<String, serde_json::Error> {
        #[derive(Serialize)]
        struct TrajectoryMetadata<'a> {
            name: &'a str,
            id: &'a str,
            max_iterations: usize,
            neighbourhood: Vec<Direction>,
            payoff_matrix: &'a PayoffMatrix,
            grid: GridMetadata<'a>
        }

        #[derive(Serialize)]
        struct GridMetadata<'a> {
            num_rows: usize,
            num_cols: usize,
            wrapped: bool,
            rng_settings: &'a Option<RngSettings>
        }

        let metadata = TrajectoryMetadata {
            name: &self.name,
            id: &self.id,
            max_iterations: self.max_iterations,
            neighbourhood: self.neighbourhood.get_directions(),
            payoff_matrix: &self.payoff.matrix,
            grid: GridMetadata {
                num_rows: self.grid.num_rows,
                num_cols: self.grid.num_cols,
                wrapped: self.grid.wrapped,
                rng_settings: &self.grid.rng_settings
            }
        };

        serde_json::to_string_pretty(&metadata)
    }
}
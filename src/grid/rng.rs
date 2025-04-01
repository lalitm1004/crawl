use rand::{Rng, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngSettings {
    pub seed: u64,
    pub cooperator_frequency: f64,
}

impl RngSettings {
    pub fn new(seed: Option<u64>, cooperator_frequency: f64) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&cooperator_frequency) {
            return Err("cooperator_frequence must lie between 0.0 and 1.0".to_string());
        }

        let seed = seed.unwrap_or_else(|| {
            let mut rng = ThreadRng::default();
            // escape expected keyword with r#
            rng.r#gen::<u64>()
        });

        Ok(Self {
            seed,
            cooperator_frequency,
        })
    }
}

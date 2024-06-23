use super::Generator;
use rand::Rng;

pub use crate::domain_layer::data::entity::raffle::Raffle_Seed;

impl Generator<Raffle_Seed> {
    pub fn generate() -> String {
        return rand::thread_rng().gen::<f64>().to_string();
    }
}

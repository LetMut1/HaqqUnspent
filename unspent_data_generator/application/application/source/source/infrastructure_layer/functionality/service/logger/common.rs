use super::Logger;
pub use crate::{
    domain_layer::data::entity::task::Task,
    infrastructure_layer::data::control_type::Common,
};
use tracing::info;

impl Logger<Common> {
    pub fn log_info<'a>(message: &'a str) -> () {
        info!(
            "{}",
            message,
        );

        return ();
    }
}

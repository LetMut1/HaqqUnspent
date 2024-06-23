use super::Resolver;
pub use crate::infrastructure_layer::data::control_type::UTCDateTime;
use chrono::Utc;

impl Resolver<UTCDateTime> {
    pub fn get_now() -> i64 {
        return Utc::now().timestamp();
    }
}

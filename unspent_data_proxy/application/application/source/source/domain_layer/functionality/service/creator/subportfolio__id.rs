use super::Creator;
pub use crate::domain_layer::data::entity::subportfolio::Subportfolio_Id;
use uuid::Uuid;

impl Creator<Subportfolio_Id> {
    pub fn create() -> Subportfolio_Id {
        return Subportfolio_Id(Uuid::new_v4().to_string());
    }

    pub fn create_minimum_length() -> String {
        return Self::STRING_MINIMUM_LENGTH_VALUE;
    }
}

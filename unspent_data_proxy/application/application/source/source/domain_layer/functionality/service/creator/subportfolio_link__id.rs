use super::Creator;
pub use crate::domain_layer::data::entity::subportfolio_link::SubportfolioLink_Id;
use uuid::Uuid;

impl Creator<SubportfolioLink_Id> {
    pub fn create() -> String {
        return Uuid::new_v4().to_string();
    }
}

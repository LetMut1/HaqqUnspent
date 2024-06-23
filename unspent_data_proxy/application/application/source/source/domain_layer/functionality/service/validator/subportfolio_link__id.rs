use super::Validator;
pub use crate::domain_layer::data::entity::subportfolio_link::SubportfolioLink_Id;
use uuid::Uuid;

impl Validator<SubportfolioLink_Id> {
    pub fn is_valid<'a>(subportfolio_link_id: &'a str) -> bool {
        if let Err(_) = Uuid::parse_str(subportfolio_link_id) {
            return false;
        }

        return true;
    }
}

use super::Validator;
pub use crate::domain_layer::data::entity::subportfolio::Subportfolio_Id;
use uuid::Uuid;

impl Validator<Subportfolio_Id> {
    pub fn is_valid<'a>(subportfolio_id: &'a str) -> bool {
        if let Err(_) = Uuid::parse_str(subportfolio_id) {
            return false;
        }

        return true;
    }
}

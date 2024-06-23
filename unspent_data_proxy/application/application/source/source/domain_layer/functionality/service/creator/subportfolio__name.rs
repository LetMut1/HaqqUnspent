use super::Creator;
pub use crate::domain_layer::data::entity::subportfolio::Subportfolio_Name;

impl Creator<Subportfolio_Name> {
    pub fn create_minimum_length() -> Subportfolio_Name {
        return Subportfolio_Name(Self::STRING_MINIMUM_LENGTH_VALUE);
    }
}

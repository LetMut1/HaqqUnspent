use super::Validator;
pub use crate::domain_layer::data::entity::subportfolio::Subportfolio_Description;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        error::Error,
    },
    functionality::service::regular_expression_applicator::RegularExpressionApplicator,
};

impl Validator<Subportfolio_Description> {
    pub fn is_valid<'a>(subportfolio_description: &'a str) -> Result<bool, Auditor<Error>> {
        let subportfolio_description_ = match RegularExpressionApplicator::remove_extra_spaces(subportfolio_description) {
            Ok(subportfolio_description__) => subportfolio_description__,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        let subportfolio_description__ = subportfolio_description_.as_str();

        return Ok(
            subportfolio_description == subportfolio_description__
                && subportfolio_description.len() <= Subportfolio_Description::MAXIMUM_BYTES_QUANTITY
                && !subportfolio_description.is_empty(),
        );
    }
}

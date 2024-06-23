use super::Validator;
pub use crate::domain_layer::data::entity::subportfolio_link::SubportfolioLink_Description;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        error::Error,
    },
    functionality::service::regular_expression_applicator::RegularExpressionApplicator,
};

impl Validator<SubportfolioLink_Description> {
    pub fn is_valid<'a>(subportfolio_link_description: &'a str) -> Result<bool, Auditor<Error>> {
        let subportfolio_link_description_ = match RegularExpressionApplicator::remove_extra_spaces(subportfolio_link_description) {
            Ok(subportfolio_link_description__) => subportfolio_link_description__,
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

        let subportfolio_link_description__ = subportfolio_link_description_.as_str();

        return Ok(
            subportfolio_link_description == subportfolio_link_description__
                && subportfolio_link_description.len() <= SubportfolioLink_Description::MAXIMUM_BYTES_QUANTITY
                && !subportfolio_link_description.is_empty(),
        );
    }
}

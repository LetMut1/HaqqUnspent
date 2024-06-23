use super::Validator;
pub use crate::domain_layer::data::entity::subportfolio::Subportfolio_Name;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        error::Error,
    },
    functionality::service::regular_expression_applicator::RegularExpressionApplicator,
};

impl Validator<Subportfolio_Name> {
    pub fn is_valid<'a>(subportfolio_name: &'a str) -> Result<bool, Auditor<Error>> {
        let subportfolio_name_ = match RegularExpressionApplicator::remove_extra_spaces(subportfolio_name) {
            Ok(subportfolio_name__) => subportfolio_name__,
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

        let subportfolio_name__ = subportfolio_name_.as_str();

        return Ok(subportfolio_name == subportfolio_name__ && subportfolio_name.len() <= Subportfolio_Name::MAXIMUM_BYTES_QUANTITY && !subportfolio_name.is_empty());
    }
}

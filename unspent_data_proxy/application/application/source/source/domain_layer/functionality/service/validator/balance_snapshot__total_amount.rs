use super::Validator;
pub use crate::domain_layer::data::entity::balance_snapshot::BalanceSnapshot_TotalAmount;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        error::Error,
    },
    functionality::service::regular_expression_applicator::RegularExpressionApplicator,
};

impl Validator<BalanceSnapshot_TotalAmount> {
    pub fn is_valid<'a>(balance_snapshot_total_amount: &'a BalanceSnapshot_TotalAmount) -> Result<bool, Auditor<Error>> {
        let is_float_number = match RegularExpressionApplicator::is_float_number(balance_snapshot_total_amount.0.as_str()) {
            Ok(is_float_number_) => is_float_number_,
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

        let balance_snapshot_total_amount_ = balance_snapshot_total_amount.0.as_str();

        return Ok(
            is_float_number
                && balance_snapshot_total_amount_ != "0"
                && balance_snapshot_total_amount_ != "0.0"
                && balance_snapshot_total_amount_ != ".0"
                && !balance_snapshot_total_amount_.contains('-'),
        );
    }
}

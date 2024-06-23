use super::Validator;
pub use crate::domain_layer::data::entity::base_balance_snapshot::BaseBalanceSnapshot_Amount;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        error::Error,
    },
    functionality::service::regular_expression_applicator::RegularExpressionApplicator,
};

impl Validator<BaseBalanceSnapshot_Amount> {
    pub fn is_valid<'a>(base_balance_snapshot_amount: &'a BaseBalanceSnapshot_Amount) -> Result<bool, Auditor<Error>> {
        let is_float_number = match RegularExpressionApplicator::is_float_number(base_balance_snapshot_amount.0.as_str()) {
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

        let base_balance_snapshot_amount_ = base_balance_snapshot_amount.0.as_str();

        return Ok(
            is_float_number
                && base_balance_snapshot_amount_ != "0"
                && base_balance_snapshot_amount_ != "0.0"
                && base_balance_snapshot_amount_ != ".0"
                && !base_balance_snapshot_amount_.contains('-'),
        );
    }
}

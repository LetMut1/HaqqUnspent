use super::Validator;
pub use crate::domain_layer::data::entity::aggregated_balance_snapshot::AggregatedBalanceSnapshot_TotalAmount;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        error::Error,
    },
    functionality::service::regular_expression_applicator::RegularExpressionApplicator,
};

impl Validator<AggregatedBalanceSnapshot_TotalAmount> {
    pub fn is_valid<'a>(aggregated_balance_snapshot_total_amount: &'a AggregatedBalanceSnapshot_TotalAmount) -> Result<bool, Auditor<Error>> {
        let is_float_number = match RegularExpressionApplicator::is_float_number(aggregated_balance_snapshot_total_amount.0.as_str()) {
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

        let aggregated_balance_snapshot_total_amount_ = aggregated_balance_snapshot_total_amount.0.as_str();

        return Ok(
            is_float_number
                && aggregated_balance_snapshot_total_amount_ != "0"
                && aggregated_balance_snapshot_total_amount_ != "0.0"
                && aggregated_balance_snapshot_total_amount_ != ".0"
                && !aggregated_balance_snapshot_total_amount_.contains('-'),
        );
    }
}

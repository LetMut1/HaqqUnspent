use super::Validator;
pub use crate::domain_layer::data::entity::asset_snapshot::AssetSnapshot_PriceUsd;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        error::Error,
    },
    functionality::service::regular_expression_applicator::RegularExpressionApplicator,
};

impl Validator<AssetSnapshot_PriceUsd> {
    pub fn is_valid<'a>(asset_snapshot_price_usd: &'a AssetSnapshot_PriceUsd) -> Result<bool, Auditor<Error>> {
        let is_float_number = match RegularExpressionApplicator::is_float_number(asset_snapshot_price_usd.0.as_str()) {
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

        let asset_snapshot_price_usd_ = asset_snapshot_price_usd.0.as_str();

        return Ok(
            is_float_number
                && asset_snapshot_price_usd_ != "0"
                && asset_snapshot_price_usd_ != "0.0"
                && asset_snapshot_price_usd_ != ".0"
                && !asset_snapshot_price_usd_.contains('-'),
        );
    }
}

use super::{
    by::{
        By3,
        By6,
    },
    queried::BalanceSnapshot,
    ClickhouseRepository,
};
use crate::{
    domain_layer::data::entity::subportfolio_base_balance_snapshot_for_over_366_days::SubportfolioBaseBalanceSnapshotForOver366Days,
    infrastructure_layer::data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        error::Error,
    },
};
use clickhouse::Client;

impl ClickhouseRepository<SubportfolioBaseBalanceSnapshotForOver366Days> {
    pub async fn find_history<'a>(clickhouse_client: &'a Client, by_6: &'a By6<'_>) -> Result<Vec<BalanceSnapshot>, Auditor<Error>> {
        let subportfolio_base_balance_snapshot_registry = match Self::find_subportfolio_base_balance_snapshot_history_(
            clickhouse_client,
            by_6,
            "unspentio.subportfolio_base_balance_snapshot_for_over_366_days",
            "unspentio.asset_snapshot_for_over_366_days",
        )
        .await
        {
            Ok(subportfolio_base_balance_snapshot_registry_) => subportfolio_base_balance_snapshot_registry_,
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

        return Ok(subportfolio_base_balance_snapshot_registry);
    }

    pub async fn lightweight_delete<'a>(clickhouse_client: &'a Client, by_3: &'a By3<'_>) -> Result<(), Auditor<Error>> {
        if let Err(mut error_auditor) = Self::lightweight_delete_subportfolio_base_balance_snapshot_(
            clickhouse_client,
            by_3,
            "unspentio.subportfolio_base_balance_snapshot_for_over_366_days",
        )
        .await
        {
            error_auditor.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                ),
            );

            return Err(error_auditor);
        };

        return Ok(());
    }
}

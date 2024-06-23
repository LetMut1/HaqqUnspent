use super::{
    by::By1,
    queried::BalanceSnapshot,
    ClickhouseRepository,
};
use crate::{
    domain_layer::data::entity::balance_snapshot_for_366_days::BalanceSnapshotFor366Days,
    infrastructure_layer::data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        error::Error,
    },
};
use clickhouse::Client;

impl ClickhouseRepository<BalanceSnapshotFor366Days> {
    pub async fn find_history<'a>(clickhouse_client: &'a Client, by_1: &'a By1<'_>) -> Result<Vec<BalanceSnapshot>, Auditor<Error>> {
        let balance_snapshot_registry = match Self::find_balance_snapshot_history_(
            clickhouse_client,
            by_1,
            "unspentio.balance_snapshot_for_366_days",
            "unspentio.asset_snapshot_for_366_days",
        )
        .await
        {
            Ok(balance_snapshot_registry_) => balance_snapshot_registry_,
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

        return Ok(balance_snapshot_registry);
    }
}

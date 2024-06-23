use super::{
    by::By3,
    ClickhouseRepository,
};
use crate::{
    domain_layer::data::entity::subportfolio_base_balance_snapshot::SubportfolioBaseBalanceSnapshot,
    infrastructure_layer::data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        error::{
            Error,
            Other,
            Runtime,
        },
    },
};
use clickhouse::{
    self,
    Client,
};

impl ClickhouseRepository<SubportfolioBaseBalanceSnapshot> {
    pub async fn find_minimum_date_for_user_and_subportfolio<'a>(clickhouse_client: &'a Client, by_3: &'a By3<'_>) -> Result<Option<u32>, Auditor<Error>> {
        let query = " \
            SELECT \
                min(sbbs.created_at) AS ca \
            FROM \
                unspentio.subportfolio_base_balance_snapshot sbbs \
            WHERE \
                sbbs.user_id = ? \
                AND sbbs.subportfolio_id = ? \
            GROUP BY \
                sbbs.user_id, \
                sbbs.subportfolio_id \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let mut row_cursor = match clickhouse_client.query(query).bind(by_3.user_id).bind(by_3.subportfolio_id).fetch::<u32>() {
            Ok(row_cursor_) => row_cursor_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let subportfolio_base_balance_snapshot_created_at = match row_cursor.next().await {
            Ok(subportfolio_base_balance_snapshot_created_at_) => subportfolio_base_balance_snapshot_created_at_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        return Ok(subportfolio_base_balance_snapshot_created_at);
    }

    pub async fn lightweight_delete<'a>(clickhouse_client: &'a Client, by_3: &'a By3<'_>) -> Result<(), Auditor<Error>> {
        if let Err(mut error_auditor) = Self::lightweight_delete_subportfolio_base_balance_snapshot_(
            clickhouse_client,
            by_3,
            "unspentio.subportfolio_base_balance_snapshot",
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

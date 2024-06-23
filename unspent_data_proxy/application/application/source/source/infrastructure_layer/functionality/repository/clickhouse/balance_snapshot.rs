use super::{
    by::By2,
    ClickhouseRepository,
};
use crate::{
    domain_layer::data::entity::balance_snapshot::BalanceSnapshot,
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

impl ClickhouseRepository<BalanceSnapshot> {
    pub async fn create<'a>(clickhouse_client: &'a Client, balance_snapshot_registry: &'a [BalanceSnapshot]) -> Result<(), Auditor<Error>> {
        if balance_snapshot_registry.is_empty() {
            return Ok(());
        }

        let mut query = "\
            INSERT INTO unspentio.balance_snapshot \
            ( \
                user_id, \
                asset_id, \
                total_amount, \
                created_at \
            ) \
            VALUES"
            .to_string();

        '_a: for balance_snapshot in balance_snapshot_registry.iter() {
            query = format!(
                "{} \
                ( \
                    {}, \
                    ?, \
                    toDecimal128(?, 19), \
                    fromUnixTimestamp({}) \
                ),",
                query, balance_snapshot.user_id.0, balance_snapshot.created_at.0
            );
        }

        let mut query_ = clickhouse_client.query(query.as_str());

        '_a: for balance_snapshot in balance_snapshot_registry.iter() {
            query_ = query_.bind(balance_snapshot.asset_id.0.as_str()).bind(balance_snapshot.total_amount.0.as_str());
        }

        if let Err(error) = query_.execute().await {
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

        return Ok(());
    }

    pub async fn find_minimum_date_for_user<'a>(clickhouse_client: &'a Client, by_2: By2) -> Result<Option<u32>, Auditor<Error>> {
        let query = " \
            SELECT \
                min(bs.created_at) AS ca \
            FROM \
                unspentio.balance_snapshot bs \
            WHERE \
                bs.user_id = ? \
            GROUP BY \
                bs.user_id \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let query_ = clickhouse_client.query(query).bind(by_2.user_id);

        let mut row_cursor = match query_.fetch::<u32>() {
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

        let balance_snapshot_created_at = match row_cursor.next().await {
            Ok(balance_snapshot_created_at_) => balance_snapshot_created_at_,
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

        return Ok(balance_snapshot_created_at);
    }
}

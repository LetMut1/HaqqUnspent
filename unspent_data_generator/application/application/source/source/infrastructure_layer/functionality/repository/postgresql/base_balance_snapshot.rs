use super::PostgresqlRepository;
use crate::{
    domain_layer::data::entity::base_balance_snapshot::BaseBalanceSnapshot_1,
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
use tokio_postgres::{
    types::ToSql,
    Client,
    Row,
};

impl PostgresqlRepository<BaseBalanceSnapshot_1> {
    pub async fn get<'a>(client: &'a Client, limit: i64, offset: i64) -> Result<Vec<Row>, Auditor<Error>> {
        let query = format!(
            "\
            SELECT \
                b.user_id AS ui, \
                b.asset_id AS ai, \
                b.exchange_id AS ei, \
                b.wallet_id AS wi, \
                b.chain_id AS ci, \
                b.network AS ne, \
                CASE \
                    WHEN b.balance IS NOT NULL \
                    THEN b.balance::TEXT \
                    ELSE NULL \
                END AS b, \
                w.address AS a, \
                w.label AS l, \
                e.name AS na \
            FROM \
                balances b \
            INNER JOIN \
                assets a \
            ON \
                b.asset_id = a.id \
                AND b.asset_id IS NOT NULL \
            INNER JOIN \
                users u \
            ON \
                b.user_id = u.id \
                AND b.user_id IS NOT NULL \
            LEFT OUTER JOIN \
                wallets w \
            ON \
                b.wallet_id = w.id \
                AND b.user_id = w.user_id \
                AND b.asset_id = w.asset_id \
                AND b.wallet_id IS NOT NULL \
                AND b.user_id IS NOT NULL \
                AND b.asset_id IS NOT NULL \
            LEFT OUTER JOIN \
                exchanges e \
            ON \
                b.exchange_id = e.id \
                AND b.exchange_id IS NOT NULL \
            WHERE \
                b.balance IS NOT NULL \
                AND b.balance != double precision 'NaN' \
                AND b.balance != 0 \
            ORDER BY \
                b.id ASC \
            LIMIT {} \
            OFFSET {};",
            limit, offset,
        );

        let row_registry = match client
            .query(
                query.as_str(),
                ([] as [&(dyn ToSql + Sync); 0]).as_slice(),
            )
            .await
        {
            Ok(row_registry_) => row_registry_,
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

        return Ok(row_registry);
    }
}

use super::PostgresqlRepository;
use crate::{
    domain_layer::data::entity::aggregated_balance_snapshot::AggregatedBalanceSnapshot_1,
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

impl PostgresqlRepository<AggregatedBalanceSnapshot_1> {
    pub async fn get<'a>(client: &'a Client, limit: i64, offset: i64) -> Result<Vec<Row>, Auditor<Error>> {
        let query = format!(
            "\
            SELECT \
                b.user_id AS ui, \
                b.asset_id AS ai, \
                sum(b.balance)::TEXT AS b \
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
            WHERE \
                b.balance IS NOT NULL \
                AND b.balance != double precision 'NaN' \
                AND b.balance != 0 \
            GROUP BY \
                b.user_id, \
                b.asset_id \
            ORDER BY \
                b.user_id ASC \
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

pub mod aggregated_balance_snapshot;
pub mod asset;
pub mod asset_snapshot;
pub mod base_balance_snapshot;

pub use crate::infrastructure_layer::data::control_type::Common;
use crate::infrastructure_layer::data::{
    auditor::Auditor,
    backtrace::BacktracePart,
    error::{
        Error,
        Other,
        Runtime,
    },
};
use std::marker::PhantomData;
use tokio_postgres::{
    types::ToSql,
    Client,
    Row,
};

pub struct PostgresqlRepository<E> {
    _entity: PhantomData<E>,
}

impl PostgresqlRepository<Common> {
    pub async fn get_wallet_asset_registry<'a>(client: &'a Client, user_id: i32, wallet_id: i32) -> Result<Vec<Row>, Auditor<Error>> {
        let query = format!(
            "\
            SELECT \
                b.asset_id AS ai, \
                b.chain_id AS ci, \
                b.network AS n, \
                w.a AS a, \
                w.l AS l \
            FROM \
                balances b \
            INNER JOIN (\
                SELECT \
                    w.address AS a, \
                    w.label AS l \
                FROM \
                    wallets w \
                WHERE \
                    w.id = {}\
            ) AS w \
            WHERE \
                b.user_id = {} \
                AND b.balance IS NOT NULL \
                AND b.balance != double precision 'NaN' \
                AND b.balance != 0;",
            wallet_id, user_id,
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

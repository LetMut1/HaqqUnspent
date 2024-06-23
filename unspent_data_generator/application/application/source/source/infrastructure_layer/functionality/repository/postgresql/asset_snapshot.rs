use super::PostgresqlRepository;
use crate::{
    domain_layer::data::entity::asset_snapshot::AssetSnapshot_1,
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

impl PostgresqlRepository<AssetSnapshot_1> {
    pub async fn get<'a>(client: &'a Client, limit: i64, offset: i64) -> Result<Vec<Row>, Auditor<Error>> {
        let query = format!(
            "\
            SELECT \
                a.id AS i, \
                a.price_usd::TEXT AS pu, \
                CASE \
                    WHEN a.price_btc IS NOT NULL \
                    THEN a.price_btc::TEXT \
                    ELSE NULL \
                END AS pb \
            FROM \
                assets a \
            WHERE \
                a.price_usd IS NOT NULL \
            ORDER BY \
                a.id ASC \
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

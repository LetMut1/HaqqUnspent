use super::ClickhouseRepository;
use crate::{
    domain_layer::data::entity::asset_snapshot::AssetSnapshot,
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

impl ClickhouseRepository<AssetSnapshot> {
    pub async fn create<'a>(clickhouse_client: &'a Client, asset_snapshot_registry: &'a [AssetSnapshot]) -> Result<(), Auditor<Error>> {
        if asset_snapshot_registry.is_empty() {
            return Ok(());
        }

        let mut parameter_registry: Vec<&'_ str> = vec![];

        let mut query = "\
            INSERT INTO unspentio.asset_snapshot \
            ( \
                asset_id, \
                price_usd, \
                price_btc, \
                created_at \
            ) \
            VALUES"
            .to_string();

        '_a: for asset_snapshot in asset_snapshot_registry.iter() {
            match asset_snapshot.price_btc {
                Some(ref price_btc_) => {
                    query = format!(
                        "{} \
                        ( \
                            ?, \
                            toDecimal128(?, 19), \
                            toDecimal128(?, 19), \
                            fromUnixTimestamp({}) \
                        ),",
                        query, asset_snapshot.created_at.0,
                    );

                    parameter_registry.push(asset_snapshot.asset_id.0.as_str());

                    parameter_registry.push(asset_snapshot.price_usd.0.as_str());

                    parameter_registry.push(price_btc_.0.as_str());
                }
                None => {
                    query = format!(
                        "{} \
                        ( \
                            ?, \
                            toDecimal128(?, 19), \
                            NULL, \
                            fromUnixTimestamp({})\
                        ),",
                        query, asset_snapshot.created_at.0,
                    );

                    parameter_registry.push(asset_snapshot.asset_id.0.as_str());

                    parameter_registry.push(asset_snapshot.price_usd.0.as_str());
                }
            }
        }

        let mut query_ = clickhouse_client.query(query.as_str());

        '_a: for parameter in parameter_registry.into_iter() {
            query_ = query_.bind(parameter);
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
}

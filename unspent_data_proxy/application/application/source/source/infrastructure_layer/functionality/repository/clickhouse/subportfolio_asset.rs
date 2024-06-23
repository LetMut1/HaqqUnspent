use super::{
    by::By3,
    ClickhouseRepository,
};
use crate::{
    domain_layer::data::entity::subportfolio_asset::{
        SubportfolioAsset,
        SubportfolioAsset_1,
        SubportfolioAsset_2,
    },
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
    query::RowCursor,
    Client,
};

impl ClickhouseRepository<SubportfolioAsset> {
    pub async fn create<'a>(clickhouse_client: &'a Client, subportfolio_asset_registry: &'a [SubportfolioAsset]) -> Result<(), Auditor<Error>> {
        if subportfolio_asset_registry.is_empty() {
            return Ok(());
        }

        let mut query = "\
            INSERT INTO unspentio.subportfolio_asset \
            ( \
                user_id, \
                subportfolio_id, \
                exchange_id, \
                exchange_name, \
                wallet_id, \
                wallet_address, \
                wallet_label, \
                asset_network, \
                asset_chain_id, \
                asset_id, \
                created_at, \
                updated_at, \
                is_deleted \
            ) \
            VALUES"
            .to_string();

        for _ in subportfolio_asset_registry {
            query = format!(
                "{} ( \
                    ?, \
                    ?, \
                    ?, \
                    ?, \
                    ?, \
                    ?, \
                    ?, \
                    ?, \
                    ?, \
                    ?, \
                    fromUnixTimestamp(?), \
                    fromUnixTimestamp(?), \
                    ? \
                ),",
                query,
            );
        }

        let mut query_ = clickhouse_client.query(query.as_str());

        for subportfolio_asset in subportfolio_asset_registry {
            query_ = query_
                .bind(subportfolio_asset.user_id)
                .bind(&subportfolio_asset.subportfolio_id)
                .bind(&subportfolio_asset.exchange_id)
                .bind(&subportfolio_asset.exchange_name)
                .bind(&subportfolio_asset.wallet_id)
                .bind(&subportfolio_asset.wallet_address)
                .bind(&subportfolio_asset.wallet_label)
                .bind(&subportfolio_asset.asset_network)
                .bind(subportfolio_asset.asset_chain_id)
                .bind(&subportfolio_asset.asset_id)
                .bind(subportfolio_asset.created_at)
                .bind(subportfolio_asset.updated_at)
                .bind(subportfolio_asset.is_deleted);
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

    pub async fn delete<'a>(clickhouse_client: &'a Client, by_3: &'a By3<'_>) -> Result<(), Auditor<Error>> {
        let query = "\
            INSERT INTO unspentio.subportfolio_asset \
            (\
                user_id, \
                subportfolio_id, \
                exchange_id, \
                exchange_name, \
                wallet_id, \
                wallet_address, \
                wallet_label, \
                asset_network, \
                asset_chain_id, \
                asset_id, \
                created_at, \
                updated_at, \
                is_deleted\
            ) \
            SELECT \
                sa.user_id AS ui, \
                sa.subportfolio_id AS si, \
                sa.exchange_id AS ei, \
                sa.exchange_name AS en, \
                sa.wallet_id AS wi, \
                sa.wallet_address AS wa, \
                sa.wallet_label AS wl, \
                sa.asset_network AS an, \
                sa.asset_chain_id AS aci, \
                sa.asset_id AS ai, \
                sa.created_at AS ca, \
                now('UTC') AS ua, \
                1 AS id \
            FROM \
                unspentio.subportfolio_asset sa \
            FINAL \
            WHERE \
                sa.user_id = ? \
                AND sa.subportfolio_id = ? \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let query_ = clickhouse_client.query(query).bind(by_3.user_id).bind(by_3.subportfolio_id);

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

impl ClickhouseRepository<SubportfolioAsset_1> {
    pub async fn get_all<'a>(clickhouse_client: &'a Client, by_3: &'a By3<'_>) -> Result<Vec<SubportfolioAsset_1>, Auditor<Error>> {
        let query = "\
            SELECT \
                sa.exchange_id AS ei, \
                sa.wallet_id AS wi, \
                sa.asset_network AS an, \
                sa.asset_chain_id as aci, \
                sa.asset_id AS ai \
            FROM \
                unspentio.subportfolio_asset sa \
            FINAL \
            WHERE \
                sa.user_id = ? \
                AND sa.subportfolio_id = ? \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let query_ = clickhouse_client.query(query).bind(by_3.user_id).bind(by_3.subportfolio_id);

        let mut subportfolio_asset_1_row_cursor = match query_.fetch::<SubportfolioAsset_1>() {
            Ok(subportfolio_asset_1_row_cursor_) => subportfolio_asset_1_row_cursor_,
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

        let mut subportfolio_asset_1_registry: Vec<SubportfolioAsset_1> = vec![];

        'a: loop {
            match subportfolio_asset_1_row_cursor.next().await {
                Ok(subportfolio_asset_1) => {
                    match subportfolio_asset_1 {
                        Some(subportfolio_asset_1_) => {
                            subportfolio_asset_1_registry.push(subportfolio_asset_1_);
                        }
                        None => {
                            break 'a;
                        }
                    }
                }
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
            }
        }

        return Ok(subportfolio_asset_1_registry);
    }
}

impl ClickhouseRepository<SubportfolioAsset_2> {
    pub fn get_all<'a>(clickhouse_client: &'a Client, by_3: &'a By3<'_>) -> Result<RowCursor<SubportfolioAsset_2>, Auditor<Error>> {
        let query = "\
            SELECT \
                sa.exchange_id AS ei, \
                sa.exchange_name AS en, \
                sa.wallet_id AS wi, \
                sa.wallet_address AS wa, \
                sa.wallet_label AS wl, \
                sa.asset_network AS an, \
                sa.asset_chain_id as aci, \
                sa.asset_id AS ai, \
                sa.created_at AS ca \
            FROM \
                unspentio.subportfolio_asset sa \
            FINAL \
            WHERE \
                sa.user_id = ? \
                AND sa.subportfolio_id = ? \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let query_ = clickhouse_client.query(query).bind(by_3.user_id).bind(by_3.subportfolio_id);

        let subportfolio_asset_2_row_cursor = match query_.fetch::<SubportfolioAsset_2>() {
            Ok(subportfolio_asset_2_row_cursor_) => subportfolio_asset_2_row_cursor_,
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

        return Ok(subportfolio_asset_2_row_cursor);
    }
}

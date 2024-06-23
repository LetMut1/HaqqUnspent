use super::{
    by::{
        By11,
        By3,
    },
    ClickhouseRepository,
};
use crate::{
    domain_layer::data::entity::subportfolio_trackable_wallet::{
        SubportfolioTrackableWallet,
        SubportfolioTrackableWallet_1,
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
    Row,
};
use serde::{
    Deserialize,
    Serialize,
};

impl ClickhouseRepository<SubportfolioTrackableWallet> {
    pub async fn create<'a>(clickhouse_client: &'a Client, subportfolio_trackable_wallet_registry: &'a [SubportfolioTrackableWallet]) -> Result<(), Auditor<Error>> {
        if subportfolio_trackable_wallet_registry.is_empty() {
            return Ok(());
        }

        let mut query = "\
            INSERT INTO unspentio.subportfolio_trackable_wallet \
            ( \
                wallet_id, \
                user_id, \
                subportfolio_id, \
                created_at, \
                updated_at, \
                is_deleted \
            ) \
            VALUES"
            .to_string();

        for _ in subportfolio_trackable_wallet_registry {
            query = format!(
                "{} ( \
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

        for subportfolio_trackable_wallet in subportfolio_trackable_wallet_registry {
            query_ = query_
                .bind(&subportfolio_trackable_wallet.wallet_id)
                .bind(subportfolio_trackable_wallet.user_id)
                .bind(&subportfolio_trackable_wallet.subportfolio_id)
                .bind(subportfolio_trackable_wallet.created_at)
                .bind(subportfolio_trackable_wallet.updated_at)
                .bind(subportfolio_trackable_wallet.is_deleted);
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
            INSERT INTO unspentio.subportfolio_trackable_wallet \
            (\
                wallet_id, \
                user_id, \
                subportfolio_id, \
                created_at, \
                updated_at, \
                is_deleted\
            ) \
            SELECT \
                stw.wallet_id AS wi, \
                stw.user_id AS ui, \
                stw.subportfolio_id AS si, \
                stw.created_at AS ca, \
                now('UTC') AS ua, \
                1 AS id \
            FROM \
                unspentio.subportfolio_trackable_wallet stw \
            FINAL \
            WHERE \
                stw.user_id = ? \
                AND stw.subportfolio_id = ? \
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

    pub async fn get<'a>(clickhouse_client: &'a Client, by_11: By11, limit: i16) -> Result<Vec<SubportfolioTrackableWalletAggregated>, Auditor<Error>> {
        let user_id = match by_11.user_id {
            Some(user_id_) => user_id_,
            None => i32::MIN,
        };

        let query = format!(
            "SELECT \
                stw.user_id AS ui, \
                stw.subportfolio_id AS si, \
                groupArray(stw.wallet_id) AS gawi \
            FROM \
                unspentio.subportfolio_trackable_wallet stw \
            FINAL \
            WHERE \
                stw.user_id >= {} \
            GROUP BY \
                stw.user_id, \
                stw.subportfolio_id \
            ORDER BY \
                stw.user_id ASC \
            LIMIT {} \
            SETTINGS \
                optimize_move_to_prewhere = 0",
            user_id,
            limit,
        );

        let query_ = clickhouse_client.query(query.as_str());

        let subportfolio_trackable_wallet_aggregated_registry = match query_.fetch_all::<SubportfolioTrackableWalletAggregated>().await {
            Ok(subportfolio_trackable_wallet_aggregated_registry_) => subportfolio_trackable_wallet_aggregated_registry_,
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

        return Ok(subportfolio_trackable_wallet_aggregated_registry);
    }
}

impl ClickhouseRepository<SubportfolioTrackableWallet_1> {
    pub fn get_<'a>(clickhouse_client: &'a Client, by_3: &'a By3<'_>) -> Result<RowCursor<SubportfolioTrackableWallet_1>, Auditor<Error>> {
        let query = "\
            SELECT \
                stw.wallet_id AS wi \
            FROM \
                unspentio.subportfolio_trackable_wallet stw \
            FINAL \
            WHERE \
                stw.user_id = ? \
                AND stw.subportfolio_id = ? \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let query_ = clickhouse_client.query(query).bind(by_3.user_id).bind(by_3.subportfolio_id);

        let subportfolio_trackable_wallet_1_row_cursor = match query_.fetch::<SubportfolioTrackableWallet_1>() {
            Ok(subportfolio_trackable_wallet_1_row_cursor_) => subportfolio_trackable_wallet_1_row_cursor_,
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

        return Ok(subportfolio_trackable_wallet_1_row_cursor);
    }

    pub async fn get<'a>(clickhouse_client: &'a Client, by_3: &'a By3<'_>) -> Result<Vec<SubportfolioTrackableWallet_1>, Auditor<Error>> {
        let mut subportfolio_trackable_wallet_1_row_cursor = match Self::get_(
            clickhouse_client,
            by_3,
        ) {
            Ok(subportfolio_trackable_wallet_1_row_cursor_) => subportfolio_trackable_wallet_1_row_cursor_,
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

        let mut subportfolio_trackable_wallet_1_registry: Vec<SubportfolioTrackableWallet_1> = vec![];

        'a: loop {
            match subportfolio_trackable_wallet_1_row_cursor.next().await {
                Ok(subportfolio_trackable_wallet_1) => {
                    match subportfolio_trackable_wallet_1 {
                        Some(subportfolio_trackable_wallet_1_) => {
                            subportfolio_trackable_wallet_1_registry.push(subportfolio_trackable_wallet_1_);
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

        return Ok(subportfolio_trackable_wallet_1_registry);
    }
}

#[derive(Row, Serialize, Deserialize)]
pub struct SubportfolioTrackableWalletAggregated {
    user_id: i32,
    subportfolio_id: String,
    wallet_id_registry: Vec<i32>,
}

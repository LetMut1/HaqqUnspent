use super::{
    by::{
        By10,
        By5,
    },
    queried::AssetSnapshotHistory,
    ClickhouseRepository,
};
use crate::{
    domain_layer::data::entity::asset_snapshot_for_7_days::AssetSnapshotFor7Days,
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
    Client,
    Row,
};
use serde::Deserialize;
use std::collections::HashMap;

impl ClickhouseRepository<AssetSnapshotFor7Days> {
    pub async fn find_history<'a>(clickhouse_client: &'a Client, by_5: &'a By5<'_>) -> Result<Vec<AssetSnapshotHistory>, Auditor<Error>> {
        let asset_snapshot_history_registry = match Self::find_asset_snapshot_history_(
            clickhouse_client,
            by_5,
            "unspentio.asset_snapshot_for_7_days",
        )
        .await
        {
            Ok(asset_snapshot_history_registry_) => asset_snapshot_history_registry_,
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

        return Ok(asset_snapshot_history_registry);
    }

    pub async fn find_history_for_price_difference_percentage_calculating_for_24_hours<'a>(
        clickhouse_client: &'a Client,
        by_10: &'a By10<'_>,
    ) -> Result<HashMap<String, AssetSnapshotForPriceDifferencePercentageCalculatingFor24Hours>, Auditor<Error>> {
        #[derive(Row, Deserialize)]
        struct AssetSnapshotForPriceDifferencePercentageCalculatingFor24Hours_ {
            asset_id: String,
            price_usd: String,
            price_btc: String,
            is_null_price_btc: u8,
        }

        let query = "\
            WITH \
                now('UTC') AS now, \
                (now - INTERVAL 24 HOUR) AS date_24_hours_, \
                CASE \
                    WHEN dateDiff('minute', toStartOfInterval(date_24_hours_, INTERVAL 1 HOUR, 'UTC') AS start_of_interval_date_24_hours, date_24_hours_, 'UTC') < 30 \
                    THEN start_of_interval_date_24_hours \
                    ELSE start_of_interval_date_24_hours + INTERVAL 1 HOUR \
                END AS date_24_hours \
            SELECT \
                asf7d.asset_id AS ai, \
                cast(asf7d.price_usd, 'String') AS pu, \
                CASE \
                    WHEN isNull(asf7d.price_btc) \
                    THEN '' \
                    ELSE cast(asf7d.price_btc, 'String') \
                END AS pb, \
                isNull(asf7d.price_btc) AS inpb \
            FROM \
                unspentio.asset_snapshot_for_7_days asf7d \
            FINAL \
            WHERE \
                asf7d.created_at = date_24_hours \
                AND asf7d.asset_id IN ? \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let asset_id_registry_chunks = by_10.asset_id_registry.chunks(25);

        let mut asset_snapshot_for_price_difference_percentage_calculating_for_24_hours_hash_nap =
            HashMap::<String, AssetSnapshotForPriceDifferencePercentageCalculatingFor24Hours>::new();

        '_a: for asset_id_registry in asset_id_registry_chunks {
            let query_ = clickhouse_client.query(query).bind(asset_id_registry);

            let mut row_cursor = match query_.fetch::<AssetSnapshotForPriceDifferencePercentageCalculatingFor24Hours_>() {
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

            'b: loop {
                let asset_snapshot_for_price_difference_percentage_for_24_hours_calculating = match row_cursor.next().await {
                    Ok(asset_snapshot_for_price_difference_percentage_calculating_for_24_hours_) => asset_snapshot_for_price_difference_percentage_calculating_for_24_hours_,
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

                let asset_snapshot_for_price_difference_percentage_calculating_for_24_hours_ = match asset_snapshot_for_price_difference_percentage_for_24_hours_calculating {
                    Some(asset_snapshot_for_price_difference_percentage_calculating_for_24_hours__) => asset_snapshot_for_price_difference_percentage_calculating_for_24_hours__,
                    None => {
                        break 'b;
                    }
                };

                let asset_snapshot_for_price_difference_percentage_calculating_for_24_hours_price_btc =
                    if asset_snapshot_for_price_difference_percentage_calculating_for_24_hours_.is_null_price_btc == 1 {
                        None
                    } else {
                        Some(asset_snapshot_for_price_difference_percentage_calculating_for_24_hours_.price_btc)
                    };

                asset_snapshot_for_price_difference_percentage_calculating_for_24_hours_hash_nap.insert(
                    asset_snapshot_for_price_difference_percentage_calculating_for_24_hours_.asset_id,
                    AssetSnapshotForPriceDifferencePercentageCalculatingFor24Hours {
                        price_usd: asset_snapshot_for_price_difference_percentage_calculating_for_24_hours_.price_usd,
                        price_btc: asset_snapshot_for_price_difference_percentage_calculating_for_24_hours_price_btc,
                    },
                );
            }
        }

        return Ok(asset_snapshot_for_price_difference_percentage_calculating_for_24_hours_hash_nap);
    }

    pub async fn find_history_for_price_difference_percentage_calculating<'a>(
        clickhouse_client: &'a Client,
        by_10: &'a By10<'_>,
    ) -> Result<HashMap<String, AssetSnapshotForPriceDifferencePercentageCalculatingFor7Days>, Auditor<Error>> {
        #[derive(Row, Deserialize)]
        struct AssetSnapshotForPriceDifferencePercentageCalculatingFor7Days_ {
            asset_id: String,
            price_usd: String,
        }

        let query = "\
            WITH \
                now('UTC') AS now, \
                (now - INTERVAL 7 DAY) AS date_7_days_, \
                CASE \
                    WHEN dateDiff('minute', toStartOfInterval(date_7_days_, INTERVAL 1 HOUR, 'UTC') AS start_of_interval_date_7_days, date_7_days_, 'UTC') < 30 \
                    THEN start_of_interval_date_7_days \
                    ELSE start_of_interval_date_7_days + INTERVAL 1 HOUR \
                END AS date_7_days \
            SELECT \
                asf7d.asset_id AS ai, \
                cast(asf7d.price_usd, 'String') AS pu \
            FROM \
                unspentio.asset_snapshot_for_7_days asf7d \
            FINAL \
            WHERE \
                asf7d.created_at = date_7_days \
                AND asf7d.asset_id IN ? \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let asset_id_registry_chunks = by_10.asset_id_registry.chunks(25);

        let mut asset_snapshot_for_price_difference_percentage_calculating_for_7_days_hash_map =
            HashMap::<String, AssetSnapshotForPriceDifferencePercentageCalculatingFor7Days>::new();

        '_a: for asset_id_registry in asset_id_registry_chunks {
            let query_ = clickhouse_client.query(query).bind(asset_id_registry);

            let mut row_cursor = match query_.fetch::<AssetSnapshotForPriceDifferencePercentageCalculatingFor7Days_>() {
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

            'b: loop {
                let asset_snapshot_for_price_difference_percentage_calculating_for_7_days = match row_cursor.next().await {
                    Ok(asset_snapshot_for_price_difference_percentage_calculating_for_7_days_) => asset_snapshot_for_price_difference_percentage_calculating_for_7_days_,
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

                let asset_snapshot_for_price_difference_percentage_calculating_for_7_days_ = match asset_snapshot_for_price_difference_percentage_calculating_for_7_days {
                    Some(asset_snapshot_for_price_difference_percentage_calculating_for_7_days__) => asset_snapshot_for_price_difference_percentage_calculating_for_7_days__,
                    None => {
                        break 'b;
                    }
                };

                asset_snapshot_for_price_difference_percentage_calculating_for_7_days_hash_map.insert(
                    asset_snapshot_for_price_difference_percentage_calculating_for_7_days_.asset_id,
                    AssetSnapshotForPriceDifferencePercentageCalculatingFor7Days {
                        price_usd: asset_snapshot_for_price_difference_percentage_calculating_for_7_days_.price_usd,
                    },
                );
            }
        }

        return Ok(asset_snapshot_for_price_difference_percentage_calculating_for_7_days_hash_map);
    }
}

pub struct AssetSnapshotForPriceDifferencePercentageCalculatingFor24Hours {
    pub price_usd: String,
    pub price_btc: Option<String>,
}

pub struct AssetSnapshotForPriceDifferencePercentageCalculatingFor7Days {
    pub price_usd: String,
}

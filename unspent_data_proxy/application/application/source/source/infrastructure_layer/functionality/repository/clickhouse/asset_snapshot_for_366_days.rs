use super::{
    by::{
        By10,
        By5,
    },
    queried::AssetSnapshotHistory,
    ClickhouseRepository,
};
use crate::{
    domain_layer::data::entity::asset_snapshot_for_366_days::AssetSnapshotFor366Days,
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

impl ClickhouseRepository<AssetSnapshotFor366Days> {
    pub async fn find_history<'a>(clickhouse_client: &'a Client, by_5: &'a By5<'_>) -> Result<Vec<AssetSnapshotHistory>, Auditor<Error>> {
        let asset_snapshot_history_registry = match Self::find_asset_snapshot_history_(
            clickhouse_client,
            by_5,
            "unspentio.asset_snapshot_for_366_days",
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

    pub async fn find_history_for_price_difference_percentage_calculating<'a>(
        clickhouse_client: &'a Client,
        by_10: &'a By10<'_>,
    ) -> Result<HashMap<String, AssetSnapshotForPriceDifferencePercentageCalculatingFor366Days>, Auditor<Error>> {
        #[derive(Row, Deserialize)]
        struct AssetSnapshotForPriceDifferencePercentageCalculatingFor366Days_ {
            asset_id: String,
            price_usd: String,
        }

        let query = "\
            WITH \
                now('UTC') AS now, \
                (now - INTERVAL 1 YEAR) AS date_1_year_, \
                CASE \
                    WHEN dateDiff('hour', toStartOfInterval(date_1_year_, INTERVAL 1 DAY, 'UTC') AS start_of_interval_date_1_year, date_1_year_, 'UTC') <= 12 \
                    THEN start_of_interval_date_1_year \
                    ELSE start_of_interval_date_1_year + INTERVAL 1 DAY \
                END AS date_1_year__, \
                CASE \
                    WHEN toDayOfWeek(date_1_year__, 0, 'UTC') AS day_of_week_date_1_year <= 2 \
                    THEN (date_1_year__ - INTERVAL (day_of_week_date_1_year - 1) DAY) \
                    WHEN day_of_week_date_1_year <= 4 \
                    THEN (date_1_year__ + INTERVAL (4 - day_of_week_date_1_year) DAY) \
                    WHEN day_of_week_date_1_year = 5 \
                    THEN (date_1_year__ - INTERVAL 1 DAY) \
                    WHEN day_of_week_date_1_year <= 7 \
                    THEN (date_1_year__ + INTERVAL (7 - day_of_week_date_1_year + 1) DAY) \
                    ELSE date_1_year__ \
                END AS date_1_year \
            SELECT \
                asf366d.asset_id AS ai, \
                cast(asf366d.price_usd, 'String') AS pu \
            FROM \
                unspentio.asset_snapshot_for_366_days asf366d \
            FINAL \
            WHERE \
                asf366d.created_at = date_1_year \
                AND asf366d.asset_id IN ? \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let asset_id_registry_chunks = by_10.asset_id_registry.chunks(25);

        let mut asset_snapshot_for_price_difference_percentage_calculating_for_366_days_hash_map =
            HashMap::<String, AssetSnapshotForPriceDifferencePercentageCalculatingFor366Days>::new();

        '_a: for asset_id_registry in asset_id_registry_chunks {
            let query_ = clickhouse_client.query(query).bind(asset_id_registry);

            let mut row_cursor = match query_.fetch::<AssetSnapshotForPriceDifferencePercentageCalculatingFor366Days_>() {
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
                let asset_snapshot_for_price_difference_percentage_calculating_for_366_days = match row_cursor.next().await {
                    Ok(asset_snapshot_for_price_difference_percentage_calculating_for_366_days_) => asset_snapshot_for_price_difference_percentage_calculating_for_366_days_,
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

                let asset_snapshot_for_price_difference_percentage_calculating_for_366_days_ = match asset_snapshot_for_price_difference_percentage_calculating_for_366_days {
                    Some(asset_snapshot_for_price_difference_percentage_calculating_for_366_days__) => asset_snapshot_for_price_difference_percentage_calculating_for_366_days__,
                    None => {
                        break 'b;
                    }
                };

                asset_snapshot_for_price_difference_percentage_calculating_for_366_days_hash_map.insert(
                    asset_snapshot_for_price_difference_percentage_calculating_for_366_days_.asset_id,
                    AssetSnapshotForPriceDifferencePercentageCalculatingFor366Days {
                        price_usd: asset_snapshot_for_price_difference_percentage_calculating_for_366_days_.price_usd,
                    },
                );
            }
        }

        return Ok(asset_snapshot_for_price_difference_percentage_calculating_for_366_days_hash_map);
    }
}

#[derive(Row, Deserialize)]
pub struct AssetSnapshotForPriceDifferencePercentageCalculatingFor366Days {
    pub price_usd: String,
}

use super::{
    by::{
        By10,
        By5,
    },
    queried::AssetSnapshotHistory,
    ClickhouseRepository,
};
use crate::{
    domain_layer::data::entity::asset_snapshot_for_30_days::AssetSnapshotFor30Days,
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

impl ClickhouseRepository<AssetSnapshotFor30Days> {
    pub async fn find_history<'a>(clickhouse_client: &'a Client, by_5: &'a By5<'_>) -> Result<Vec<AssetSnapshotHistory>, Auditor<Error>> {
        let asset_snapshot_history_registry = match Self::find_asset_snapshot_history_(
            clickhouse_client,
            by_5,
            "unspentio.asset_snapshot_for_30_days",
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
    ) -> Result<HashMap<String, AssetSnapshotForPriceDifferencePercentageCalculatingFor30Days>, Auditor<Error>> {
        #[derive(Row, Deserialize)]
        struct AssetSnapshotForPriceDifferencePercentageCalculatingFor30Days_ {
            asset_id: String,
            price_usd: String,
        }

        let query = "\
            WITH \
                now('UTC') AS now, \
                (now - INTERVAL 30 DAY) AS date_30_days_, \
                CASE \
                    WHEN dateDiff('hour', toStartOfInterval(date_30_days_, INTERVAL 12 HOUR, 'UTC') AS start_of_interval_date_30_days, date_30_days_, 'UTC') <= 6 \
                    THEN start_of_interval_date_30_days \
                    ELSE start_of_interval_date_30_days + INTERVAL 12 HOUR \
                END AS date_30_days \
            SELECT \
                asf30d.asset_id AS ai, \
                cast(asf30d.price_usd, 'String') AS pu \
            FROM \
                unspentio.asset_snapshot_for_30_days asf30d \
            FINAL \
            WHERE \
                asf30d.created_at = date_30_days \
                AND asf30d.asset_id IN ? \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let asset_id_registry_chunks = by_10.asset_id_registry.chunks(25);

        let mut asset_snapshot_for_price_difference_percentage_calculating_for_30_days_hash_map =
            HashMap::<String, AssetSnapshotForPriceDifferencePercentageCalculatingFor30Days>::new();

        '_a: for asset_id_registry in asset_id_registry_chunks {
            let query_ = clickhouse_client.query(query).bind(asset_id_registry);

            let mut row_cursor = match query_.fetch::<AssetSnapshotForPriceDifferencePercentageCalculatingFor30Days_>() {
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
                let asset_snapshot_for_price_difference_percentage_calculating_for_30_days = match row_cursor.next().await {
                    Ok(asset_snapshot_for_price_difference_percentage_calculating_for_30_days_) => asset_snapshot_for_price_difference_percentage_calculating_for_30_days_,
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

                let asset_snapshot_for_price_difference_percentage_calculating_for_30_days_ = match asset_snapshot_for_price_difference_percentage_calculating_for_30_days {
                    Some(asset_snapshot_for_price_difference_percentage_calculatingfor_30_days__) => asset_snapshot_for_price_difference_percentage_calculatingfor_30_days__,
                    None => {
                        break 'b;
                    }
                };

                asset_snapshot_for_price_difference_percentage_calculating_for_30_days_hash_map.insert(
                    asset_snapshot_for_price_difference_percentage_calculating_for_30_days_.asset_id,
                    AssetSnapshotForPriceDifferencePercentageCalculatingFor30Days {
                        price_usd: asset_snapshot_for_price_difference_percentage_calculating_for_30_days_.price_usd,
                    },
                );
            }
        }

        return Ok(asset_snapshot_for_price_difference_percentage_calculating_for_30_days_hash_map);
    }
}

pub struct AssetSnapshotForPriceDifferencePercentageCalculatingFor30Days {
    pub price_usd: String,
}

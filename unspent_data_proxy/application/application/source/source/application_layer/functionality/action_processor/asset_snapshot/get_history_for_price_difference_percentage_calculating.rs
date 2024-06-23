pub use crate::infrastructure_layer::data::control_type::AssetSnapshot___GetHistoryForPriceDifferencePercentageCalculating;
use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::data::entity::{
        asset_snapshot_for_30_days::AssetSnapshotFor30Days,
        asset_snapshot_for_366_days::AssetSnapshotFor366Days,
        asset_snapshot_for_7_days::AssetSnapshotFor7Days,
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            control_type::ServerAccessToken,
            error::Error,
            invalid_argument::InvalidArgument,
            invalid_argument_result::InvalidArgumentResult,
            void::Void,
        },
        functionality::{
            repository::clickhouse::{
                by::By10,
                ClickhouseRepository,
            },
            service::validator::Validator,
        },
    },
};
use clickhouse::Client;
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashSet;

impl ActionProcessor<AssetSnapshot___GetHistoryForPriceDifferencePercentageCalculating> {
    pub async fn process(incoming: Option<Incoming>, clickhouse_client: Client) -> Result<InvalidArgumentResult<UnifiedReport<Outcoming, Void>>, Auditor<Error>> {
        let incoming_ = match incoming {
            Some(incoming__) => incoming__,
            None => {
                return Err(
                    Auditor::<Error>::new(
                        Error::create_incoming_invalid_state(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        if !Validator::<ServerAccessToken>::is_valid(&incoming_.server_access_token) {
            return Ok(
                InvalidArgumentResult::InvalidArgumentAuditor {
                    invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                        InvalidArgument::new(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                },
            );
        }

        if incoming_.asset_id_registry.is_empty() {
            return Ok(
                InvalidArgumentResult::InvalidArgumentAuditor {
                    invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                        InvalidArgument::new(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                },
            );
        }

        let mut asset_id_hash_set = HashSet::<&'_ str>::new();

        '_a: for asset_id in incoming_.asset_id_registry.as_slice() {
            if !asset_id_hash_set.insert(asset_id.as_str()) {
                return Ok(
                    InvalidArgumentResult::InvalidArgumentAuditor {
                        invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                            InvalidArgument::new(),
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    },
                );
            }
        }

        let mut asset_snapshot_for_price_difference_percentage_calculating_registry: Vec<AssetSnapshotForPriceDifferencePercentageCalculating> = vec![];

        let mut asset_snapshot_for_price_difference_percentage_calculating_for_24_hours_hash_map =
            match ClickhouseRepository::<AssetSnapshotFor7Days>::find_history_for_price_difference_percentage_calculating_for_24_hours(
                &clickhouse_client,
                &By10 {
                    asset_id_registry: incoming_.asset_id_registry.as_slice(),
                },
            )
            .await
            {
                Ok(asset_snapshot_for_price_difference_percentage_calculating_for_24_hours_hash_map) => {
                    asset_snapshot_for_price_difference_percentage_calculating_for_24_hours_hash_map
                }
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

        let mut asset_snapshot_for_price_difference_percentage_calculating_for_7_days_hash_map =
            match ClickhouseRepository::<AssetSnapshotFor7Days>::find_history_for_price_difference_percentage_calculating(
                &clickhouse_client,
                &By10 {
                    asset_id_registry: incoming_.asset_id_registry.as_slice(),
                },
            )
            .await
            {
                Ok(asset_snapshot_for_price_difference_percentage_calculating_for_7_days_hash_map) => {
                    asset_snapshot_for_price_difference_percentage_calculating_for_7_days_hash_map
                }
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

        let mut asset_snapshot_for_price_difference_percentage_calculating_for_30_days_hash_map =
            match ClickhouseRepository::<AssetSnapshotFor30Days>::find_history_for_price_difference_percentage_calculating(
                &clickhouse_client,
                &By10 {
                    asset_id_registry: incoming_.asset_id_registry.as_slice(),
                },
            )
            .await
            {
                Ok(asset_snapshot_for_price_difference_percentage_calculating_for_30_days_hash_map) => {
                    asset_snapshot_for_price_difference_percentage_calculating_for_30_days_hash_map
                }
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

        let mut asset_snapshot_for_price_difference_percentage_calculating_for_366_days_hash_map =
            match ClickhouseRepository::<AssetSnapshotFor366Days>::find_history_for_price_difference_percentage_calculating(
                &clickhouse_client,
                &By10 {
                    asset_id_registry: incoming_.asset_id_registry.as_slice(),
                },
            )
            .await
            {
                Ok(asset_snapshot_for_price_difference_percentage_calculating_for_366_days_hash_map) => {
                    asset_snapshot_for_price_difference_percentage_calculating_for_366_days_hash_map
                }
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

        'a: for asset_id in incoming_.asset_id_registry.into_iter() {
            let (asset_snapshot_for_price_difference_percentage_calculating_price_usd_24_hours, asset_snapshot_for_price_difference_percentage_calculating_price_btc_24_hours) =
                match asset_snapshot_for_price_difference_percentage_calculating_for_24_hours_hash_map.remove(&asset_id) {
                    Some(asset_snapshot_for_price_difference_percentage_calculating_for_24_hours) => {
                        (
                            asset_snapshot_for_price_difference_percentage_calculating_for_24_hours.price_usd,
                            asset_snapshot_for_price_difference_percentage_calculating_for_24_hours.price_btc,
                        )
                    }
                    None => {
                        continue 'a;
                    }
                };

            let asset_snapshot_for_price_difference_percentage_calculating_price_usd_for_7_days =
                match asset_snapshot_for_price_difference_percentage_calculating_for_7_days_hash_map.remove(&asset_id) {
                    Some(asset_snapshot_for_price_difference_percentage_calculating_for_7_days) => {
                        Some(asset_snapshot_for_price_difference_percentage_calculating_for_7_days.price_usd)
                    }
                    None => None,
                };

            let asset_snapshot_for_price_difference_percentage_calculating_price_usd_for_30_days =
                match asset_snapshot_for_price_difference_percentage_calculating_for_30_days_hash_map.remove(&asset_id) {
                    Some(asset_snapshot_for_price_difference_percentage_calculating_for_30_days) => {
                        Some(asset_snapshot_for_price_difference_percentage_calculating_for_30_days.price_usd)
                    }
                    None => None,
                };

            let asset_snapshot_for_price_difference_percentage_calculating_price_usd_for_366_days =
                match asset_snapshot_for_price_difference_percentage_calculating_for_366_days_hash_map.remove(&asset_id) {
                    Some(asset_snapshot_for_price_difference_percentage_calculating_for_366_days) => {
                        Some(asset_snapshot_for_price_difference_percentage_calculating_for_366_days.price_usd)
                    }
                    None => None,
                };

            asset_snapshot_for_price_difference_percentage_calculating_registry.push(
                AssetSnapshotForPriceDifferencePercentageCalculating {
                    asset_id,
                    price_usd_24_hours: asset_snapshot_for_price_difference_percentage_calculating_price_usd_24_hours,
                    price_btc_24_hours: asset_snapshot_for_price_difference_percentage_calculating_price_btc_24_hours,
                    price_usd_7_days: asset_snapshot_for_price_difference_percentage_calculating_price_usd_for_7_days,
                    price_usd_30_days: asset_snapshot_for_price_difference_percentage_calculating_price_usd_for_30_days,
                    price_usd_1_year: asset_snapshot_for_price_difference_percentage_calculating_price_usd_for_366_days,
                },
            )
        }

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::filled(
                    Outcoming {
                        asset_snapshot_for_price_difference_percentage_calculating_registry,
                    },
                ),
            },
        );
    }
}

#[derive(Deserialize)]
pub struct Incoming {
    server_access_token: ServerAccessToken,
    asset_id_registry: Vec<String>,
}

#[derive(Serialize)]
pub struct Outcoming {
    asset_snapshot_for_price_difference_percentage_calculating_registry: Vec<AssetSnapshotForPriceDifferencePercentageCalculating>,
}

#[derive(Serialize)]
pub struct AssetSnapshotForPriceDifferencePercentageCalculating {
    asset_id: String,
    price_usd_24_hours: String,
    price_btc_24_hours: Option<String>,
    price_usd_7_days: Option<String>,
    price_usd_30_days: Option<String>,
    price_usd_1_year: Option<String>,
}

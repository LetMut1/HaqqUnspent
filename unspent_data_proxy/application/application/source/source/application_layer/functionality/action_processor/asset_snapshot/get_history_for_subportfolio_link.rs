pub use crate::infrastructure_layer::data::control_type::AssetSnapshot___GetHistoryForSubportfolioLink;
use crate::{
    application_layer::{
        data::unified_report::{
            CommonPrecedent,
            UnifiedReport,
        },
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::{
            asset_snapshot_for_30_days::AssetSnapshotFor30Days,
            asset_snapshot_for_366_days::AssetSnapshotFor366Days,
            asset_snapshot_for_7_days::AssetSnapshotFor7Days,
            subportfolio_link::{
                SubportfolioLink_4,
                SubportfolioLink_Id,
            },
        },
        functionality::service::{
            resolver::Resolver,
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            control_type::{
                SnapshotRange,
                UTCDateTime,
            },
            error::Error,
            invalid_argument::InvalidArgument,
            invalid_argument_result::InvalidArgumentResult,
        },
        functionality::{
            repository::clickhouse::{
                by::{
                    By5,
                    By9,
                },
                queried::{
                    AssetData,
                    AssetSnapshotHistory,
                },
                ClickhouseRepository,
            },
            service::resolver::Resolver as Resolver_,
        },
        macro_rules::r#enum,
    },
};
use clickhouse::Client;
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashSet;

impl ActionProcessor<AssetSnapshot___GetHistoryForSubportfolioLink> {
    const QUANTITY_OF_SECONDS_FOR_SUBTRACTION: u32 = 5;

    pub async fn process(incoming: Option<Incoming>, clickhouse_client: Client) -> Result<InvalidArgumentResult<UnifiedReport<Outcoming, Precedent>>, Auditor<Error>> {
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

        if !Validator::<SubportfolioLink_Id>::is_valid(incoming_.subportfolio_link_id.as_str()) {
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

        let subportfolio_link_4 = match ClickhouseRepository::<SubportfolioLink_4>::find(
            &clickhouse_client,
            &By9 {
                subportfolio_link_id: incoming_.subportfolio_link_id.as_str(),
            },
        )
        .await
        {
            Ok(subportfolio_link_4_) => subportfolio_link_4_,
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

        let subportfolio_link_4_ = match subportfolio_link_4 {
            Some(subportfolio_4__) => subportfolio_4__,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::SubportfolioLink_DoesNotExist),
                    },
                );
            }
        };

        if !subportfolio_link_4_.is_active {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::SubportfolioLink_IsNotActive),
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

        let asset_snapshot_history_registry = if incoming_.range == Resolver::<SnapshotRange>::REGISTRY[0] {
            let asset_snapshot_history_registry_ = match ClickhouseRepository::<AssetSnapshotFor7Days>::find_history(
                &clickhouse_client,
                &By5 {
                    asset_id_registry: incoming_.asset_id_registry.as_slice(),
                    subtracted_quantity_of_hours: 24,
                },
            )
            .await
            {
                Ok(asset_snapshot_history_registry__) => asset_snapshot_history_registry__,
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

            let asset_snapshot_history_registry__ = match Self::normalize_asset_snapshot_history_timestamp_points_scale(
                asset_snapshot_history_registry_,
                60 * 60 * 24,
                60 * 60,
            ) {
                Ok(asset_snapshot_history_registry___) => asset_snapshot_history_registry___,
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

            asset_snapshot_history_registry__
        } else {
            let asset_snapshot_history_registry_ = if incoming_.range == Resolver::<SnapshotRange>::REGISTRY[1] {
                let asset_snapshot_history_registry__ = match ClickhouseRepository::<AssetSnapshotFor7Days>::find_history(
                    &clickhouse_client,
                    &By5 {
                        asset_id_registry: incoming_.asset_id_registry.as_slice(),
                        subtracted_quantity_of_hours: 24 * 7,
                    },
                )
                .await
                {
                    Ok(asset_snapshot_history_registry___) => asset_snapshot_history_registry___,
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

                let asset_snapshot_history_registry___ = match Self::normalize_asset_snapshot_history_timestamp_points_scale(
                    asset_snapshot_history_registry__,
                    60 * 60 * 24 * 7,
                    60 * 60,
                ) {
                    Ok(asset_snapshot_history_registry____) => asset_snapshot_history_registry____,
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

                asset_snapshot_history_registry___
            } else {
                let asset_snapshot_history_registry__ = if incoming_.range == Resolver::<SnapshotRange>::REGISTRY[2] {
                    let asset_snapshot_history_registry___ = match ClickhouseRepository::<AssetSnapshotFor30Days>::find_history(
                        &clickhouse_client,
                        &By5 {
                            asset_id_registry: incoming_.asset_id_registry.as_slice(),
                            subtracted_quantity_of_hours: 24 * 31,
                        },
                    )
                    .await
                    {
                        Ok(asset_snapshot_history_registry____) => asset_snapshot_history_registry____,
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

                    let asset_snapshot_history_registry____ = match Self::normalize_asset_snapshot_history_timestamp_points_scale(
                        asset_snapshot_history_registry___,
                        60 * 60 * 24 * 31,
                        60 * 60 * 12,
                    ) {
                        Ok(asset_snapshot_history_registry_____) => asset_snapshot_history_registry_____,
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

                    asset_snapshot_history_registry____
                } else {
                    let asset_snapshot_history_registry___ = if incoming_.range == Resolver::<SnapshotRange>::REGISTRY[3] {
                        let asset_snapshot_history_registry____ = match ClickhouseRepository::<AssetSnapshotFor366Days>::find_history(
                            &clickhouse_client,
                            &By5 {
                                asset_id_registry: incoming_.asset_id_registry.as_slice(),
                                subtracted_quantity_of_hours: 24 * 366,
                            },
                        )
                        .await
                        {
                            Ok(asset_snapshot_history_registry_____) => asset_snapshot_history_registry_____,
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

                        let asset_snapshot_history_registry_____ = match Self::normalize_asset_snapshot_history_timestamp_points_scale(
                            asset_snapshot_history_registry____,
                            60 * 60 * 24 * 366,
                            60 * 60 * 24 * 4,
                        ) {
                            Ok(asset_snapshot_history_registry______) => asset_snapshot_history_registry______,
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

                        asset_snapshot_history_registry_____
                    } else {
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
                    };

                    asset_snapshot_history_registry___
                };

                asset_snapshot_history_registry__
            };

            asset_snapshot_history_registry_
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::filled(
                    Outcoming {
                        asset_snapshot_history_registry,
                    },
                ),
            },
        );
    }

    fn normalize_asset_snapshot_history_timestamp_points_scale(
        asset_snapshot_history_registry: Vec<AssetSnapshotHistory>,
        all_data_search_segment_in_seconds: u32,
        data_point_existence_segment_in_seconds: u32,
    ) -> Result<Vec<AssetSnapshotHistory>, Auditor<Error>> {
        let now = match Resolver_::<UTCDateTime>::get_now_() {
            Ok(now_) => now_,
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

        let mut asset_snapshot_history_registry_: Vec<AssetSnapshotHistory> = vec![];

        '_a: for asset_snapshot_history in asset_snapshot_history_registry.into_iter() {
            if !asset_snapshot_history.asset_price_usd_registry.is_empty() {
                if asset_snapshot_history.asset_price_usd_registry[0].timestamp > (now - all_data_search_segment_in_seconds + data_point_existence_segment_in_seconds) {
                    let zero_price = "0.0".to_string();

                    let mut asset_price_usd_registry: Vec<AssetData> = vec![
                        AssetData {
                            price_usd: zero_price.clone(),
                            timestamp: now - all_data_search_segment_in_seconds,
                        },
                        AssetData {
                            price_usd: zero_price,
                            timestamp: asset_snapshot_history.asset_price_usd_registry[0].timestamp - Self::QUANTITY_OF_SECONDS_FOR_SUBTRACTION,
                        },
                    ];

                    '_b: for asset_price_usd in asset_snapshot_history.asset_price_usd_registry.into_iter() {
                        asset_price_usd_registry.push(asset_price_usd)
                    }

                    asset_snapshot_history_registry_.push(
                        AssetSnapshotHistory {
                            asset_id: asset_snapshot_history.asset_id,
                            asset_price_usd_registry,
                        },
                    );
                } else {
                    asset_snapshot_history_registry_.push(asset_snapshot_history);
                }
            }
        }

        return Ok(asset_snapshot_history_registry_);
    }
}

#[derive(Deserialize)]
pub struct Incoming {
    subportfolio_link_id: String,
    range: String,
    asset_id_registry: Vec<String>,
}

#[derive(Serialize)]
pub struct Outcoming {
    asset_snapshot_history_registry: Vec<AssetSnapshotHistory>,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::SubportfolioLink_DoesNotExist,
        CommonPrecedent::SubportfolioLink_IsNotActive,
    }
);

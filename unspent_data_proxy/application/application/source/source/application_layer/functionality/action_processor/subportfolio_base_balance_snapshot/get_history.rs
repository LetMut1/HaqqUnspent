#[cfg(not(feature = "not_authorized_user"))]
use crate::infrastructure_layer::data::control_type::AccessToken;
pub use crate::infrastructure_layer::data::control_type::SubportfolioBaseBalanceSnapshot___GetHistory;
#[cfg(feature = "not_authorized_user")]
use crate::infrastructure_layer::functionality::service::resolver::access_token::User;
#[cfg(not(feature = "not_authorized_user"))]
use crate::infrastructure_layer::functionality::service::resolver::Resolver as Resolver_;
use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::{
            _remote::Asset_Id,
            subportfolio::Subportfolio_Id,
            subportfolio_base_balance_snapshot::SubportfolioBaseBalanceSnapshot,
            subportfolio_base_balance_snapshot_for_120_days::SubportfolioBaseBalanceSnapshotFor120Days,
            subportfolio_base_balance_snapshot_for_30_days::SubportfolioBaseBalanceSnapshotFor30Days,
            subportfolio_base_balance_snapshot_for_366_days::SubportfolioBaseBalanceSnapshotFor366Days,
            subportfolio_base_balance_snapshot_for_7_days::SubportfolioBaseBalanceSnapshotFor7Days,
            subportfolio_base_balance_snapshot_for_over_366_days::SubportfolioBaseBalanceSnapshotForOver366Days,
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
            control_type::SnapshotRange,
            error::Error,
            invalid_argument::InvalidArgument,
            invalid_argument_result::InvalidArgumentResult,
            void::Void,
        },
        functionality::repository::clickhouse::{
            by::{
                By3,
                By6,
            },
            queried::BalanceSnapshot as BalanceSnapshot_,
            ClickhouseRepository,
        },
    },
};
use chrono::{
    DateTime,
    NaiveDateTime,
    Utc,
};
use clickhouse::Client;
use serde::{
    Deserialize,
    Serialize,
};

impl ActionProcessor<SubportfolioBaseBalanceSnapshot___GetHistory> {
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

        let user_;

        #[cfg(feature = "not_authorized_user")]
        {
            user_ = incoming_.user;
        }

        #[cfg(not(feature = "not_authorized_user"))]
        {
            let user = match Resolver_::<AccessToken>::get_user(&incoming_.access_token).await {
                Ok(user_) => user_,
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

            user_ = match user {
                Some(user__) => user__,
                None => {
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
            };
        }

        if !Validator::<Subportfolio_Id>::is_valid(incoming_.subportfolio_id.0.as_str()) {
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

        let subportfolio_base_balance_snapshot_registry = if incoming_.range == Resolver::<SnapshotRange>::REGISTRY[0] {
            let subportfolio_base_balance_snapshot_registry_ = match ClickhouseRepository::<SubportfolioBaseBalanceSnapshotFor7Days>::find_history(
                &clickhouse_client,
                &By6 {
                    user_id: user_.id,
                    subportfolio_id: incoming_.subportfolio_id.0.as_str(),
                    reference_asset_id: incoming_.reference_asset_id.0.as_str(),
                    subtracted_quantity_of_hours: 24,
                },
            )
            .await
            {
                Ok(subportfolio_base_balance_snapshot_registry__) => subportfolio_base_balance_snapshot_registry__,
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

            subportfolio_base_balance_snapshot_registry_
        } else {
            let subportfolio_base_balance_snapshot_registry_ = if incoming_.range == Resolver::<SnapshotRange>::REGISTRY[1] {
                let subportfolio_base_balance_snapshot_registry__ = match ClickhouseRepository::<SubportfolioBaseBalanceSnapshotFor7Days>::find_history(
                    &clickhouse_client,
                    &By6 {
                        user_id: user_.id,
                        subportfolio_id: incoming_.subportfolio_id.0.as_str(),
                        reference_asset_id: incoming_.reference_asset_id.0.as_str(),
                        subtracted_quantity_of_hours: 24 * 7,
                    },
                )
                .await
                {
                    Ok(subportfolio_base_balance_snapshot_registry___) => subportfolio_base_balance_snapshot_registry___,
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

                subportfolio_base_balance_snapshot_registry__
            } else {
                let subportfolio_base_balance_snapshot_registry__ = if incoming_.range == Resolver::<SnapshotRange>::REGISTRY[2] {
                    let subportfolio_base_balance_snapshot_registry___ = match ClickhouseRepository::<SubportfolioBaseBalanceSnapshotFor30Days>::find_history(
                        &clickhouse_client,
                        &By6 {
                            user_id: user_.id,
                            subportfolio_id: incoming_.subportfolio_id.0.as_str(),
                            reference_asset_id: incoming_.reference_asset_id.0.as_str(),
                            subtracted_quantity_of_hours: 24 * 31,
                        },
                    )
                    .await
                    {
                        Ok(subportfolio_base_balance_snapshot_registry____) => subportfolio_base_balance_snapshot_registry____,
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

                    subportfolio_base_balance_snapshot_registry___
                } else {
                    let subportfolio_base_balance_snapshot_registry___ = if incoming_.range == Resolver::<SnapshotRange>::REGISTRY[3] {
                        let subportfolio_base_balance_snapshot_registry____ = match ClickhouseRepository::<SubportfolioBaseBalanceSnapshotFor366Days>::find_history(
                            &clickhouse_client,
                            &By6 {
                                user_id: user_.id,
                                subportfolio_id: incoming_.subportfolio_id.0.as_str(),
                                reference_asset_id: incoming_.reference_asset_id.0.as_str(),
                                subtracted_quantity_of_hours: 24 * 366,
                            },
                        )
                        .await
                        {
                            Ok(subportfolio_base_balance_snapshot_registry_____) => subportfolio_base_balance_snapshot_registry_____,
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

                        subportfolio_base_balance_snapshot_registry____
                    } else {
                        let subportfolio_base_balance_snapshot_registry____ = if incoming_.range == Resolver::<SnapshotRange>::REGISTRY[4] {
                            let subportfolio_base_balance_snapshot_created_at =
                                match ClickhouseRepository::<SubportfolioBaseBalanceSnapshot>::find_minimum_date_for_user_and_subportfolio(
                                    &clickhouse_client,
                                    &By3 {
                                        user_id: user_.id,
                                        subportfolio_id: incoming_.subportfolio_id.0.as_str(),
                                    },
                                )
                                .await
                                {
                                    Ok(subportfolio_base_balance_snapshot_created_at_) => subportfolio_base_balance_snapshot_created_at_,
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

                            let subportfolio_base_balance_snapshot_registry_____ = match subportfolio_base_balance_snapshot_created_at {
                                Some(subportfolio_base_balance_snapshot_created_at_) => {
                                    let naive_date_time = match NaiveDateTime::from_timestamp_opt(
                                        subportfolio_base_balance_snapshot_created_at_ as i64,
                                        0,
                                    ) {
                                        Some(naive_date_time_) => naive_date_time_,
                                        None => {
                                            return Err(
                                                Auditor::<Error>::new(
                                                    Error::Logic {
                                                        message: "Invalid timestamp.",
                                                    },
                                                    BacktracePart::new(
                                                        line!(),
                                                        file!(),
                                                    ),
                                                ),
                                            );
                                        }
                                    };

                                    let balance_snapshot_created_at__ = DateTime::<Utc>::from_utc(
                                        naive_date_time,
                                        Utc,
                                    );

                                    let now = Utc::now();

                                    if now <= balance_snapshot_created_at__ {
                                        return Err(
                                            Auditor::<Error>::new(
                                                Error::Logic {
                                                    message: "Invalid timestamp.",
                                                },
                                                BacktracePart::new(
                                                    line!(),
                                                    file!(),
                                                ),
                                            ),
                                        );
                                    }

                                    let difference_in_hours = (now - balance_snapshot_created_at__).num_hours();

                                    let by_6 = By6 {
                                        user_id: user_.id,
                                        subportfolio_id: incoming_.subportfolio_id.0.as_str(),
                                        reference_asset_id: incoming_.reference_asset_id.0.as_str(),
                                        subtracted_quantity_of_hours: difference_in_hours,
                                    };

                                    let subportfolio_base_balance_snapshot_registry______ = if difference_in_hours < ((24 * 7) as i64) {
                                        let subportfolio_base_balance_snapshot_registry_______ =
                                            match ClickhouseRepository::<SubportfolioBaseBalanceSnapshotFor7Days>::find_history(
                                                &clickhouse_client,
                                                &by_6,
                                            )
                                            .await
                                            {
                                                Ok(subportfolio_base_balance_snapshot_registry________) => subportfolio_base_balance_snapshot_registry________,
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

                                        subportfolio_base_balance_snapshot_registry_______
                                    } else {
                                        let subportfolio_base_balance_snapshot_registry_______ = if difference_in_hours < ((24 * 31) as i64) {
                                            let subportfolio_base_balance_snapshot_registry________ =
                                                match ClickhouseRepository::<SubportfolioBaseBalanceSnapshotFor30Days>::find_history(
                                                    &clickhouse_client,
                                                    &by_6,
                                                )
                                                .await
                                                {
                                                    Ok(subportfolio_base_balance_snapshot_registry_________) => subportfolio_base_balance_snapshot_registry_________,
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

                                            subportfolio_base_balance_snapshot_registry________
                                        } else {
                                            let subportfolio_base_balance_snapshot_registry________ = if difference_in_hours < ((24 * 120) as i64) {
                                                let subportfolio_base_balance_snapshot_registry_________ =
                                                    match ClickhouseRepository::<SubportfolioBaseBalanceSnapshotFor120Days>::find_history(
                                                        &clickhouse_client,
                                                        &by_6,
                                                    )
                                                    .await
                                                    {
                                                        Ok(subportfolio_base_balance_snapshot_registry__________) => subportfolio_base_balance_snapshot_registry__________,
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

                                                subportfolio_base_balance_snapshot_registry_________
                                            } else {
                                                let subportfolio_base_balance_snapshot_registry_________ = if difference_in_hours < ((24 * 366) as i64) {
                                                    let subportfolio_base_balance_snapshot_registry__________ =
                                                        match ClickhouseRepository::<SubportfolioBaseBalanceSnapshotFor366Days>::find_history(
                                                            &clickhouse_client,
                                                            &by_6,
                                                        )
                                                        .await
                                                        {
                                                            Ok(subportfolio_base_balance_snapshot_registry___________) => subportfolio_base_balance_snapshot_registry___________,
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

                                                    subportfolio_base_balance_snapshot_registry__________
                                                } else {
                                                    let subportfolio_base_balance_snapshot_registry__________ =
                                                        match ClickhouseRepository::<SubportfolioBaseBalanceSnapshotForOver366Days>::find_history(
                                                            &clickhouse_client,
                                                            &by_6,
                                                        )
                                                        .await
                                                        {
                                                            Ok(subportfolio_base_balance_snapshot_registry____) => subportfolio_base_balance_snapshot_registry____,
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

                                                    subportfolio_base_balance_snapshot_registry__________
                                                };

                                                subportfolio_base_balance_snapshot_registry_________
                                            };

                                            subportfolio_base_balance_snapshot_registry________
                                        };

                                        subportfolio_base_balance_snapshot_registry_______
                                    };

                                    subportfolio_base_balance_snapshot_registry______
                                }
                                None => Vec::<BalanceSnapshot_>::new(),
                            };

                            subportfolio_base_balance_snapshot_registry_____
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

                        subportfolio_base_balance_snapshot_registry____
                    };

                    subportfolio_base_balance_snapshot_registry___
                };

                subportfolio_base_balance_snapshot_registry__
            };

            subportfolio_base_balance_snapshot_registry_
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::filled(
                    Outcoming {
                        subportfolio_base_balance_snapshot_registry,
                    },
                ),
            },
        );
    }
}

#[cfg(feature = "not_authorized_user")]
#[derive(Deserialize)]
pub struct Incoming {
    user: User,
    subportfolio_id: Subportfolio_Id,
    reference_asset_id: Asset_Id,
    range: String,
}

#[cfg(not(feature = "not_authorized_user"))]
#[derive(Deserialize)]
pub struct Incoming {
    access_token: AccessToken,
    subportfolio_id: Subportfolio_Id,
    reference_asset_id: Asset_Id,
    range: String,
}

#[derive(Serialize)]
pub struct Outcoming {
    subportfolio_base_balance_snapshot_registry: Vec<BalanceSnapshot_>,
}

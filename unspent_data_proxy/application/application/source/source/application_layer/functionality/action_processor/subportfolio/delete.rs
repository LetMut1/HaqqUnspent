#[cfg(not(feature = "not_authorized_user"))]
use crate::infrastructure_layer::data::control_type::AccessToken;
pub use crate::infrastructure_layer::data::control_type::Subportfolio___Delete;
#[cfg(feature = "not_authorized_user")]
use crate::infrastructure_layer::functionality::service::resolver::access_token::User;
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
            _remote::User_Id,
            subportfolio::{
                IsDeleted,
                Subportfolio,
                Subportfolio_CreatedAt,
                Subportfolio_Id,
                Subportfolio_Name,
                Subportfolio_UpdatedAt,
            },
            subportfolio_asset::SubportfolioAsset,
            subportfolio_base_balance_snapshot::SubportfolioBaseBalanceSnapshot,
            subportfolio_base_balance_snapshot_for_120_days::SubportfolioBaseBalanceSnapshotFor120Days,
            subportfolio_base_balance_snapshot_for_30_days::SubportfolioBaseBalanceSnapshotFor30Days,
            subportfolio_base_balance_snapshot_for_366_days::SubportfolioBaseBalanceSnapshotFor366Days,
            subportfolio_base_balance_snapshot_for_7_days::SubportfolioBaseBalanceSnapshotFor7Days,
            subportfolio_base_balance_snapshot_for_over_366_days::SubportfolioBaseBalanceSnapshotForOver366Days,
            subportfolio_trackable_wallet::SubportfolioTrackableWallet,
        },
        functionality::service::{
            creator::Creator,
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            control_type::{
                TokioNonBlockingTask,
                UTCDateTime,
            },
            error::Error,
            invalid_argument::InvalidArgument,
            invalid_argument_result::InvalidArgumentResult,
            void::Void,
        },
        functionality::{
            repository::clickhouse::{
                by::By3,
                ClickhouseRepository,
            },
            service::{
                logger::Logger,
                resolver::Resolver,
                spawner::Spawner,
            },
        },
        macro_rules::r#enum,
    },
};
use clickhouse::Client;
use serde::Deserialize;
use tokio::time::{
    sleep,
    Duration,
};

impl ActionProcessor<Subportfolio___Delete> {
    pub async fn process(incoming: Option<Incoming>, clickhouse_client: Client) -> Result<InvalidArgumentResult<UnifiedReport<Void, Precedent>>, Auditor<Error>> {
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
            let user = match Resolver::<AccessToken>::get_user(&incoming_.access_token).await {
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

        let is_exist = match ClickhouseRepository::<Subportfolio>::is_exist_1(
            &clickhouse_client,
            &By3 {
                user_id: user_.id,
                subportfolio_id: incoming_.subportfolio_id.0.as_str(),
            },
        )
        .await
        {
            Ok(is_exist_) => is_exist_,
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

        if !is_exist {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::Subportfolio_DoesNotExist),
                },
            );
        }

        let subportfolio_updated_at = match Resolver::<UTCDateTime>::get_now_() {
            Ok(subportfolio_updated_at_) => subportfolio_updated_at_,
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

        let subportfolio = Subportfolio {
            user_id: User_Id(user_.id),
            id: incoming_.subportfolio_id.clone(),
            name: Creator::<Subportfolio_Name>::create_minimum_length(),
            description: None,
            created_at: Subportfolio_CreatedAt(subportfolio_updated_at),
            updated_at: Subportfolio_UpdatedAt(subportfolio_updated_at),
            is_deleted: IsDeleted::create_deleted(),
        };

        if let Err(mut error_auditor) = ClickhouseRepository::<Subportfolio>::create(
            &clickhouse_client,
            &subportfolio,
        )
        .await
        {
            error_auditor.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                ),
            );

            return Err(error_auditor);
        };

        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            Self::delete_subportfolio_related_data_1(
                clickhouse_client,
                user_.id,
                incoming_.subportfolio_id.0,
            ),
        );

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::empty(),
            },
        );
    }

    async fn delete_subportfolio_related_data_1(clickhouse_client: Client, user_id: i32, subportfolio_id: String) -> Result<(), Auditor<Error>> {
        let mut is_needed_to_delete_subportfolio_trackable_wallet = true;

        let mut is_needed_to_delete_subportfolio_asset = true;

        let mut is_needed_to_delete_subportfolio_base_balance_snapshot = true;

        let mut is_needed_to_delete_subportfolio_base_balance_snapshot_for_7_days = true;

        let mut is_needed_to_delete_subportfolio_base_balance_snapshot_for_30_days = true;

        let mut is_needed_to_delete_subportfolio_base_balance_snapshot_for_120_days = true;

        let mut is_needed_to_delete_subportfolio_base_balance_snapshot_for_366_days = true;

        let mut is_needed_to_delete_subportfolio_base_balance_snapshot_for_over_366_days = true;

        'a: loop {
            if let Err(error_auditor) = Self::delete_subportfolio_related_data_2(
                clickhouse_client.clone(),
                user_id,
                subportfolio_id.as_str(),
                &mut is_needed_to_delete_subportfolio_trackable_wallet,
                &mut is_needed_to_delete_subportfolio_asset,
                &mut is_needed_to_delete_subportfolio_base_balance_snapshot,
                &mut is_needed_to_delete_subportfolio_base_balance_snapshot_for_7_days,
                &mut is_needed_to_delete_subportfolio_base_balance_snapshot_for_30_days,
                &mut is_needed_to_delete_subportfolio_base_balance_snapshot_for_120_days,
                &mut is_needed_to_delete_subportfolio_base_balance_snapshot_for_366_days,
                &mut is_needed_to_delete_subportfolio_base_balance_snapshot_for_over_366_days,
            )
            .await
            {
                Logger::<Auditor<Error>>::log(&error_auditor);

                sleep(Duration::from_secs(5)).await;

                continue 'a;
            }

            break 'a;
        }

        return Ok(());
    }

    async fn delete_subportfolio_related_data_2<'a>(
        clickhouse_client: Client,
        user_id: i32,
        subportfolio_id: &'a str,
        is_needed_to_delete_subportfolio_trackable_wallet: &'a mut bool,
        is_needed_to_delete_subportfolio_asset: &'a mut bool,
        is_needed_to_delete_subportfolio_base_balance_snapshot: &'a mut bool,
        is_needed_to_delete_subportfolio_base_balance_snapshot_for_7_days: &'a mut bool,
        is_needed_to_delete_subportfolio_base_balance_snapshot_for_30_days: &'a mut bool,
        is_needed_to_delete_subportfolio_base_balance_snapshot_for_120_days: &'a mut bool,
        is_needed_to_delete_subportfolio_base_balance_snapshot_for_366_days: &'a mut bool,
        is_needed_to_delete_subportfolio_base_balance_snapshot_for_over_366_days: &'a mut bool,
    ) -> Result<(), Auditor<Error>> {
        let by_3 = By3 {
            user_id,
            subportfolio_id,
        };

        if *is_needed_to_delete_subportfolio_trackable_wallet {
            match ClickhouseRepository::<SubportfolioTrackableWallet>::delete(
                &clickhouse_client,
                &by_3,
            )
            .await
            {
                Ok(_) => {
                    *is_needed_to_delete_subportfolio_trackable_wallet = false;
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
            }
        }

        if *is_needed_to_delete_subportfolio_asset {
            match ClickhouseRepository::<SubportfolioAsset>::delete(
                &clickhouse_client,
                &by_3,
            )
            .await
            {
                Ok(_) => {
                    *is_needed_to_delete_subportfolio_asset = false;
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
            }
        }

        if *is_needed_to_delete_subportfolio_base_balance_snapshot {
            match ClickhouseRepository::<SubportfolioBaseBalanceSnapshot>::lightweight_delete(
                &clickhouse_client,
                &by_3,
            )
            .await
            {
                Ok(_) => {
                    *is_needed_to_delete_subportfolio_base_balance_snapshot = false;
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
            }
        }

        if *is_needed_to_delete_subportfolio_base_balance_snapshot_for_7_days {
            match ClickhouseRepository::<SubportfolioBaseBalanceSnapshotFor7Days>::lightweight_delete(
                &clickhouse_client,
                &by_3,
            )
            .await
            {
                Ok(_) => {
                    *is_needed_to_delete_subportfolio_base_balance_snapshot_for_7_days = false;
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
            }
        }

        if *is_needed_to_delete_subportfolio_base_balance_snapshot_for_30_days {
            match ClickhouseRepository::<SubportfolioBaseBalanceSnapshotFor30Days>::lightweight_delete(
                &clickhouse_client,
                &by_3,
            )
            .await
            {
                Ok(_) => {
                    *is_needed_to_delete_subportfolio_base_balance_snapshot_for_30_days = false;
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
            }
        }

        if *is_needed_to_delete_subportfolio_base_balance_snapshot_for_120_days {
            match ClickhouseRepository::<SubportfolioBaseBalanceSnapshotFor120Days>::lightweight_delete(
                &clickhouse_client,
                &by_3,
            )
            .await
            {
                Ok(_) => {
                    *is_needed_to_delete_subportfolio_base_balance_snapshot_for_120_days = false;
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
            }
        }

        if *is_needed_to_delete_subportfolio_base_balance_snapshot_for_366_days {
            match ClickhouseRepository::<SubportfolioBaseBalanceSnapshotFor366Days>::lightweight_delete(
                &clickhouse_client,
                &by_3,
            )
            .await
            {
                Ok(_) => {
                    *is_needed_to_delete_subportfolio_base_balance_snapshot_for_366_days = false;
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
            }
        }

        if *is_needed_to_delete_subportfolio_base_balance_snapshot_for_over_366_days {
            match ClickhouseRepository::<SubportfolioBaseBalanceSnapshotForOver366Days>::lightweight_delete(
                &clickhouse_client,
                &by_3,
            )
            .await
            {
                Ok(_) => {
                    *is_needed_to_delete_subportfolio_base_balance_snapshot_for_over_366_days = false;
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
            }
        }

        return Ok(());
    }
}

#[cfg(feature = "not_authorized_user")]
#[derive(Deserialize)]
pub struct Incoming {
    user: User,
    subportfolio_id: Subportfolio_Id,
}

#[cfg(not(feature = "not_authorized_user"))]
#[derive(Deserialize)]
pub struct Incoming {
    access_token: AccessToken,
    subportfolio_id: Subportfolio_Id,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::Subportfolio_DoesNotExist,
    }
);

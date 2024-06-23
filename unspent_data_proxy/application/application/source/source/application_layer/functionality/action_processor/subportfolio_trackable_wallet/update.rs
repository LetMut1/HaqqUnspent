#[cfg(not(feature = "not_authorized_user"))]
use crate::infrastructure_layer::data::control_type::AccessToken;
pub use crate::infrastructure_layer::data::control_type::SubportfolioTrackableWallet___Update;
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
            subportfolio::{
                IsDeleted,
                Subportfolio,
                Subportfolio_Id,
            },
            subportfolio_trackable_wallet::{
                SubportfolioTrackableWallet,
                SubportfolioTrackableWallet_1,
            },
        },
        functionality::service::validator::Validator,
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
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
            service::resolver::{
                utc_date_time::UTCDateTime,
                Resolver,
            },
        },
        macro_rules::r#enum,
    },
};
use clickhouse::Client;
use serde::Deserialize;
use std::collections::HashSet;

impl ActionProcessor<SubportfolioTrackableWallet___Update> {
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

        if !Validator::<Subportfolio_Id>::is_valid(incoming_.subportfolio_id.as_str()) {
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

        let by_3 = By3 {
            user_id: user_.id,
            subportfolio_id: incoming_.subportfolio_id.as_str(),
        };

        let is_exist = match ClickhouseRepository::<Subportfolio>::is_exist_1(
            &clickhouse_client,
            &by_3,
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

        let subportfolio_trackable_wallet_wallet_id_registry_for_creating_length = incoming_.subportfolio_trackable_wallet_wallet_id_registry_for_creating.len();

        let subportfolio_trackable_wallet_wallet_id_registry_for_deleting_length = incoming_.subportfolio_trackable_wallet_wallet_id_registry_for_deleting.len();

        let subportfolio_trackable_wallet_registry = if subportfolio_trackable_wallet_wallet_id_registry_for_creating_length == 0
            && subportfolio_trackable_wallet_wallet_id_registry_for_deleting_length == 0
        {
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
        } else {
            let subportfolio_trackable_wallet_registry_ =
                if subportfolio_trackable_wallet_wallet_id_registry_for_creating_length > 0 && subportfolio_trackable_wallet_wallet_id_registry_for_deleting_length == 0 {
                    let subportfolio_trackable_wallet_created_at = match Resolver::<UTCDateTime>::get_now_() {
                        Ok(subportfolio_trackable_wallet_created_at_) => subportfolio_trackable_wallet_created_at_,
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

                    let mut subportfolio_trackable_wallet_registry__: Vec<SubportfolioTrackableWallet> = vec![];

                    let mut subportfolio_trackable_wallet_wallet_id_for_creating_hash_set = HashSet::<i32>::new();

                    '_a: for subportfolio_trackable_wallet_wallet_id_for_creating in incoming_.subportfolio_trackable_wallet_wallet_id_registry_for_creating.into_iter() {
                        if !subportfolio_trackable_wallet_wallet_id_for_creating_hash_set.insert(subportfolio_trackable_wallet_wallet_id_for_creating) {
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

                        subportfolio_trackable_wallet_registry__.push(
                            SubportfolioTrackableWallet::new(
                                subportfolio_trackable_wallet_wallet_id_for_creating,
                                user_.id,
                                incoming_.subportfolio_id.clone(),
                                subportfolio_trackable_wallet_created_at,
                                subportfolio_trackable_wallet_created_at,
                                IsDeleted::create_not_deleted().get(),
                            ),
                        );
                    }

                    let subportfolio_trackable_wallet_1_registry = match ClickhouseRepository::<SubportfolioTrackableWallet_1>::get(
                        &clickhouse_client,
                        &by_3,
                    )
                    .await
                    {
                        Ok(subportfolio_trackable_wallet_1_registry_) => subportfolio_trackable_wallet_1_registry_,
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

                    if (subportfolio_trackable_wallet_1_registry.len() + subportfolio_trackable_wallet_wallet_id_registry_for_creating_length)
                        > SubportfolioTrackableWallet::MAXIMUM_QUANTITY_PER_USER_AND_SUBPORTFOLIO
                    {
                        return Ok(
                            InvalidArgumentResult::Ok {
                                subject: UnifiedReport::precedent(Precedent::SubportfolioTrackableWallet_MaximumQuantityPerUserAndSubportfolio),
                            },
                        );
                    }

                    '_a: for subportfolio_trackable_wallet_1 in subportfolio_trackable_wallet_1_registry.into_iter() {
                        if subportfolio_trackable_wallet_wallet_id_for_creating_hash_set.contains(&subportfolio_trackable_wallet_1.wallet_id) {
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

                    subportfolio_trackable_wallet_registry__
                } else {
                    let subportfolio_trackable_wallet_registry__ =
                        if subportfolio_trackable_wallet_wallet_id_registry_for_creating_length == 0 && subportfolio_trackable_wallet_wallet_id_registry_for_deleting_length > 0 {
                            let subportfolio_trackable_wallet_1_registry = match ClickhouseRepository::<SubportfolioTrackableWallet_1>::get(
                                &clickhouse_client,
                                &by_3,
                            )
                            .await
                            {
                                Ok(subportfolio_trackable_wallet_1_registry_) => subportfolio_trackable_wallet_1_registry_,
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

                            if subportfolio_trackable_wallet_1_registry.len() < subportfolio_trackable_wallet_wallet_id_registry_for_deleting_length {
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

                            let mut existed_subportfolio_trackable_wallet_wallet_id_hash_set = HashSet::<i32>::new();

                            '_a: for subportfolio_trackable_wallet_1 in subportfolio_trackable_wallet_1_registry.into_iter() {
                                existed_subportfolio_trackable_wallet_wallet_id_hash_set.insert(subportfolio_trackable_wallet_1.wallet_id);
                            }

                            let subportfolio_trackable_wallet_updated_at = match Resolver::<UTCDateTime>::get_now_() {
                                Ok(subportfolio_trackable_wallet_updated_at_) => subportfolio_trackable_wallet_updated_at_,
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

                            let mut subportfolio_trackable_wallet_wallet_id_for_deleting_hash_set = HashSet::<i32>::new();

                            let mut subportfolio_trackable_wallet_registry___: Vec<SubportfolioTrackableWallet> = vec![];

                            '_a: for subportfolio_trackable_wallet_wallet_id_for_deleting in incoming_.subportfolio_trackable_wallet_wallet_id_registry_for_deleting.into_iter() {
                                if !existed_subportfolio_trackable_wallet_wallet_id_hash_set.contains(&subportfolio_trackable_wallet_wallet_id_for_deleting) {
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

                                if !subportfolio_trackable_wallet_wallet_id_for_deleting_hash_set.insert(subportfolio_trackable_wallet_wallet_id_for_deleting) {
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

                                subportfolio_trackable_wallet_registry___.push(
                                    SubportfolioTrackableWallet::new(
                                        subportfolio_trackable_wallet_wallet_id_for_deleting,
                                        user_.id,
                                        incoming_.subportfolio_id.clone(),
                                        subportfolio_trackable_wallet_updated_at,
                                        subportfolio_trackable_wallet_updated_at,
                                        IsDeleted::create_deleted().get(),
                                    ),
                                );
                            }

                            subportfolio_trackable_wallet_registry___
                        } else {
                            let subportfolio_trackable_wallet_created_at = match Resolver::<UTCDateTime>::get_now_() {
                                Ok(subportfolio_trackable_wallet_created_at_) => subportfolio_trackable_wallet_created_at_,
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

                            let mut subportfolio_trackable_wallet_registry___: Vec<SubportfolioTrackableWallet> = vec![];

                            let mut subportfolio_trackable_wallet_wallet_id_for_creating_hash_set = HashSet::<i32>::new();

                            '_a: for subportfolio_trackable_wallet_wallet_id_for_creating in incoming_.subportfolio_trackable_wallet_wallet_id_registry_for_creating.into_iter() {
                                if !subportfolio_trackable_wallet_wallet_id_for_creating_hash_set.insert(subportfolio_trackable_wallet_wallet_id_for_creating) {
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

                                subportfolio_trackable_wallet_registry___.push(
                                    SubportfolioTrackableWallet::new(
                                        subportfolio_trackable_wallet_wallet_id_for_creating,
                                        user_.id,
                                        incoming_.subportfolio_id.clone(),
                                        subportfolio_trackable_wallet_created_at,
                                        subportfolio_trackable_wallet_created_at,
                                        IsDeleted::create_not_deleted().get(),
                                    ),
                                );
                            }

                            let subportfolio_trackable_wallet_1_registry = match ClickhouseRepository::<SubportfolioTrackableWallet_1>::get(
                                &clickhouse_client,
                                &by_3,
                            )
                            .await
                            {
                                Ok(subportfolio_trackable_wallet_1_registry_) => subportfolio_trackable_wallet_1_registry_,
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

                            let subportfolio_trackable_wallet_1_registry_length = subportfolio_trackable_wallet_1_registry.len();

                            if subportfolio_trackable_wallet_wallet_id_registry_for_deleting_length > subportfolio_trackable_wallet_1_registry_length {
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

                            if (subportfolio_trackable_wallet_1_registry_length + subportfolio_trackable_wallet_wallet_id_registry_for_creating_length
                                - subportfolio_trackable_wallet_wallet_id_registry_for_deleting_length)
                                > SubportfolioTrackableWallet::MAXIMUM_QUANTITY_PER_USER_AND_SUBPORTFOLIO
                            {
                                return Ok(
                                    InvalidArgumentResult::Ok {
                                        subject: UnifiedReport::precedent(Precedent::SubportfolioTrackableWallet_MaximumQuantityPerUserAndSubportfolio),
                                    },
                                );
                            }

                            let mut existed_subportfolio_trackable_wallet_wallet_id_hash_set = HashSet::<i32>::new();

                            '_a: for subportfolio_trackable_wallet_1 in subportfolio_trackable_wallet_1_registry.into_iter() {
                                existed_subportfolio_trackable_wallet_wallet_id_hash_set.insert(subportfolio_trackable_wallet_1.wallet_id);
                            }

                            let mut subportfolio_trackable_wallet_wallet_id_for_deleting_hash_set = HashSet::<i32>::new();

                            '_a: for subportfolio_trackable_wallet_wallet_id_for_deleting in incoming_.subportfolio_trackable_wallet_wallet_id_registry_for_deleting.into_iter() {
                                if !existed_subportfolio_trackable_wallet_wallet_id_hash_set.contains(&subportfolio_trackable_wallet_wallet_id_for_deleting) {
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

                                if !subportfolio_trackable_wallet_wallet_id_for_deleting_hash_set.insert(subportfolio_trackable_wallet_wallet_id_for_deleting) {
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

                                if subportfolio_trackable_wallet_wallet_id_for_creating_hash_set.contains(&subportfolio_trackable_wallet_wallet_id_for_deleting) {
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

                                subportfolio_trackable_wallet_registry___.push(
                                    SubportfolioTrackableWallet::new(
                                        subportfolio_trackable_wallet_wallet_id_for_deleting,
                                        user_.id,
                                        incoming_.subportfolio_id.clone(),
                                        subportfolio_trackable_wallet_created_at,
                                        subportfolio_trackable_wallet_created_at,
                                        IsDeleted::create_deleted().get(),
                                    ),
                                );
                            }

                            subportfolio_trackable_wallet_registry___
                        };

                    subportfolio_trackable_wallet_registry__
                };

            subportfolio_trackable_wallet_registry_
        };

        if !subportfolio_trackable_wallet_registry.is_empty() {
            if let Err(mut error_auditor) = ClickhouseRepository::<SubportfolioTrackableWallet>::create(
                &clickhouse_client,
                subportfolio_trackable_wallet_registry.as_slice(),
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
        }

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::empty(),
            },
        );
    }
}

#[cfg(feature = "not_authorized_user")]
#[derive(Deserialize)]
pub struct Incoming {
    user: User,
    subportfolio_id: String,
    subportfolio_trackable_wallet_wallet_id_registry_for_creating: Vec<i32>,
    subportfolio_trackable_wallet_wallet_id_registry_for_deleting: Vec<i32>,
}

#[cfg(not(feature = "not_authorized_user"))]
#[derive(Deserialize)]
pub struct Incoming {
    access_token: AccessToken,
    subportfolio_id: String,
    subportfolio_trackable_wallet_wallet_id_registry_for_creating: Vec<i32>,
    subportfolio_trackable_wallet_wallet_id_registry_for_deleting: Vec<i32>,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::Subportfolio_DoesNotExist,
        CommonPrecedent::SubportfolioTrackableWallet_MaximumQuantityPerUserAndSubportfolio,
    }
);

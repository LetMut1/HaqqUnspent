#[cfg(not(feature = "not_authorized_user"))]
use crate::infrastructure_layer::data::control_type::AccessToken;
pub use crate::infrastructure_layer::data::control_type::SubportfolioAsset___Update;
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
            _remote::{
                Asset_ChainId,
                Asset_Id,
                Asset_Network,
                Exchange_Id,
                Exchange_Name,
                User_Id,
                Wallet_Address,
                Wallet_Id,
                Wallet_Label,
            },
            subportfolio::{
                IsDeleted,
                Subportfolio,
                Subportfolio_Id,
            },
            subportfolio_asset::{
                SubportfolioAsset,
                SubportfolioAsset_1,
                SubportfolioAsset_CreatedAt,
                SubportfolioAsset_UpdatedAt,
            },
        },
        functionality::service::validator::Validator,
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            control_type::DefaultValue,
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
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashSet;

impl ActionProcessor<SubportfolioAsset___Update> {
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

        let by_3 = By3 {
            user_id: user_.id,
            subportfolio_id: incoming_.subportfolio_id.0.as_str(),
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

        let asset_registry_for_creating_length = incoming_.asset_registry_for_creating.len();

        let asset_registry_for_deleting_length = incoming_.asset_registry_for_deleting.len();

        let subportfolio_asset_registry = if asset_registry_for_creating_length == 0 && asset_registry_for_deleting_length == 0 {
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
            let subportfolio_asset_registry_ = if asset_registry_for_creating_length > 0 && asset_registry_for_deleting_length == 0 {
                let asset_registry_for_creating = match Self::check_and_transform_asset_registry_to_vec(incoming_.asset_registry_for_creating) {
                    InvalidArgumentResult::Ok {
                        subject,
                    } => subject,
                    InvalidArgumentResult::InvalidArgumentAuditor {
                        mut invalid_argument_auditor,
                    } => {
                        invalid_argument_auditor.add_backtrace_part(
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        );

                        return Ok(
                            InvalidArgumentResult::InvalidArgumentAuditor {
                                invalid_argument_auditor,
                            },
                        );
                    }
                };

                if let InvalidArgumentResult::InvalidArgumentAuditor {
                    mut invalid_argument_auditor,
                } = Self::check_and_transform_asset_registry_to_hash_set(asset_registry_for_creating.as_slice())
                {
                    invalid_argument_auditor.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    );

                    return Ok(
                        InvalidArgumentResult::InvalidArgumentAuditor {
                            invalid_argument_auditor,
                        },
                    );
                }

                let subportfolio_asset_1_registry = match ClickhouseRepository::<SubportfolioAsset_1>::get_all(
                    &clickhouse_client,
                    &by_3,
                )
                .await
                {
                    Ok(subportfolio_asset_1_registry_) => subportfolio_asset_1_registry_,
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

                let subportfolio_asset_quantity = subportfolio_asset_1_registry.len();

                if (subportfolio_asset_quantity + asset_registry_for_creating_length) > SubportfolioAsset::MAXIMUM_QUANTITY_PER_SUBPORTFOLIO {
                    return Ok(
                        InvalidArgumentResult::Ok {
                            subject: UnifiedReport::precedent(Precedent::SubportfolioAsset_MaximumQuantityPerSubportfolio),
                        },
                    );
                }

                let mut existed_asset_hash_set = HashSet::<(
                    &'_ Exchange_Id,
                    Wallet_Id,
                    &'_ Asset_Network,
                    Asset_ChainId,
                    &'_ Asset_Id,
                )>::new();

                '_a: for subportfolio_asset_1 in subportfolio_asset_1_registry.iter() {
                    existed_asset_hash_set.insert(
                        (
                            &subportfolio_asset_1.exchange_id,
                            subportfolio_asset_1.wallet_id,
                            &subportfolio_asset_1.asset_network,
                            subportfolio_asset_1.asset_chain_id,
                            &subportfolio_asset_1.asset_id,
                        ),
                    );
                }

                let subportfolio_asset_created_at = match Resolver::<UTCDateTime>::get_now_() {
                    Ok(subportfolio_asset_created_at_) => subportfolio_asset_created_at_,
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

                let mut subportfolio_asset_registry__: Vec<SubportfolioAsset> = vec![];

                let mut existed_asset_registry: Vec<Asset> = vec![];

                '_a: for asset in asset_registry_for_creating.into_iter() {
                    if existed_asset_hash_set.contains(
                        &(
                            &asset.exchange_id,
                            asset.wallet_id,
                            &asset.asset_network,
                            asset.asset_chain_id,
                            &asset.asset_id,
                        ),
                    ) {
                        existed_asset_registry.push(
                            Asset {
                                exchange_id: Resolver::<DefaultValue>::to_option(asset.exchange_id),
                                exchange_name: Resolver::<DefaultValue>::to_option(asset.exchange_name),
                                wallet_id: Resolver::<DefaultValue>::to_option(asset.wallet_id),
                                wallet_address: Resolver::<DefaultValue>::to_option(asset.wallet_address),
                                wallet_label: Resolver::<DefaultValue>::to_option(asset.wallet_label),
                                asset_network: Resolver::<DefaultValue>::to_option(asset.asset_network),
                                asset_chain_id: Resolver::<DefaultValue>::to_option(asset.asset_chain_id),
                                asset_id: asset.asset_id,
                            },
                        );
                    } else {
                        subportfolio_asset_registry__.push(
                            SubportfolioAsset {
                                user_id: User_Id(user_.id),
                                subportfolio_id: incoming_.subportfolio_id.clone(),
                                exchange_id: asset.exchange_id,
                                exchange_name: asset.exchange_name,
                                wallet_id: asset.wallet_id,
                                wallet_address: asset.wallet_address,
                                wallet_label: asset.wallet_label,
                                asset_network: asset.asset_network,
                                asset_chain_id: asset.asset_chain_id,
                                asset_id: asset.asset_id,
                                created_at: SubportfolioAsset_CreatedAt(subportfolio_asset_created_at),
                                updated_at: SubportfolioAsset_UpdatedAt(subportfolio_asset_created_at),
                                is_deleted: IsDeleted::create_not_deleted(),
                            },
                        );
                    }
                }

                if !existed_asset_registry.is_empty() {
                    return Ok(
                        InvalidArgumentResult::Ok {
                            subject: UnifiedReport::precedent(
                                Precedent::SubportfolioAsset_AlreadyExist {
                                    asset_registry: existed_asset_registry,
                                },
                            ),
                        },
                    );
                }

                subportfolio_asset_registry__
            } else {
                let subportfolio_asset_registry__ = if asset_registry_for_creating_length == 0 && asset_registry_for_deleting_length > 0 {
                    let asset_registry_for_deleting = match Self::check_and_transform_asset_registry_to_vec(incoming_.asset_registry_for_deleting) {
                        InvalidArgumentResult::Ok {
                            subject,
                        } => subject,
                        InvalidArgumentResult::InvalidArgumentAuditor {
                            mut invalid_argument_auditor,
                        } => {
                            invalid_argument_auditor.add_backtrace_part(
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                ),
                            );

                            return Ok(
                                InvalidArgumentResult::InvalidArgumentAuditor {
                                    invalid_argument_auditor,
                                },
                            );
                        }
                    };

                    if let InvalidArgumentResult::InvalidArgumentAuditor {
                        mut invalid_argument_auditor,
                    } = Self::check_and_transform_asset_registry_to_hash_set(asset_registry_for_deleting.as_slice())
                    {
                        invalid_argument_auditor.add_backtrace_part(
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        );

                        return Ok(
                            InvalidArgumentResult::InvalidArgumentAuditor {
                                invalid_argument_auditor,
                            },
                        );
                    }

                    let subportfolio_asset_1_registry = match ClickhouseRepository::<SubportfolioAsset_1>::get_all(
                        &clickhouse_client,
                        &by_3,
                    )
                    .await
                    {
                        Ok(subportfolio_asset_1_registry_) => subportfolio_asset_1_registry_,
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

                    let mut existed_asset_hash_set = HashSet::<(
                        &'_ Exchange_Id,
                        Wallet_Id,
                        &'_ Asset_Network,
                        Asset_ChainId,
                        &'_ Asset_Id,
                    )>::new();

                    '_a: for subportfolio_asset_1 in subportfolio_asset_1_registry.iter() {
                        existed_asset_hash_set.insert(
                            (
                                &subportfolio_asset_1.exchange_id,
                                subportfolio_asset_1.wallet_id,
                                &subportfolio_asset_1.asset_network,
                                subportfolio_asset_1.asset_chain_id,
                                &subportfolio_asset_1.asset_id,
                            ),
                        );
                    }

                    let mut not_existed_asset_registry: Vec<Asset> = vec![];

                    let mut subportfolio_asset_registry___: Vec<SubportfolioAsset> = vec![];

                    let subportfolio_asset_updated_at = match Resolver::<UTCDateTime>::get_now_() {
                        Ok(subportfolio_asset_updated_at_) => subportfolio_asset_updated_at_,
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

                    '_a: for asset in asset_registry_for_deleting.into_iter() {
                        if existed_asset_hash_set.contains(
                            &(
                                &asset.exchange_id,
                                asset.wallet_id,
                                &asset.asset_network,
                                asset.asset_chain_id,
                                &asset.asset_id,
                            ),
                        ) {
                            subportfolio_asset_registry___.push(
                                SubportfolioAsset {
                                    user_id: User_Id(user_.id),
                                    subportfolio_id: incoming_.subportfolio_id.clone(),
                                    exchange_id: asset.exchange_id,
                                    exchange_name: asset.exchange_name,
                                    wallet_id: asset.wallet_id,
                                    wallet_address: asset.wallet_address,
                                    wallet_label: asset.wallet_label,
                                    asset_network: asset.asset_network,
                                    asset_chain_id: asset.asset_chain_id,
                                    asset_id: asset.asset_id,
                                    created_at: SubportfolioAsset_CreatedAt(subportfolio_asset_updated_at),
                                    updated_at: SubportfolioAsset_UpdatedAt(subportfolio_asset_updated_at),
                                    is_deleted: IsDeleted::create_deleted(),
                                },
                            );
                        } else {
                            not_existed_asset_registry.push(
                                Asset {
                                    exchange_id: Resolver::<DefaultValue>::to_option(asset.exchange_id),
                                    exchange_name: Resolver::<DefaultValue>::to_option(asset.exchange_name),
                                    wallet_id: Resolver::<DefaultValue>::to_option(asset.wallet_id),
                                    wallet_address: Resolver::<DefaultValue>::to_option(asset.wallet_address),
                                    wallet_label: Resolver::<DefaultValue>::to_option(asset.wallet_label),
                                    asset_network: Resolver::<DefaultValue>::to_option(asset.asset_network),
                                    asset_chain_id: Resolver::<DefaultValue>::to_option(asset.asset_chain_id),
                                    asset_id: asset.asset_id,
                                },
                            );
                        }
                    }

                    if !not_existed_asset_registry.is_empty() {
                        return Ok(
                            InvalidArgumentResult::Ok {
                                subject: UnifiedReport::precedent(
                                    Precedent::SubportfolioAsset_DoesNotExist {
                                        asset_registry: not_existed_asset_registry,
                                    },
                                ),
                            },
                        );
                    }

                    subportfolio_asset_registry___
                } else {
                    let asset_registry_for_creating = match Self::check_and_transform_asset_registry_to_vec(incoming_.asset_registry_for_creating) {
                        InvalidArgumentResult::Ok {
                            subject,
                        } => subject,
                        InvalidArgumentResult::InvalidArgumentAuditor {
                            mut invalid_argument_auditor,
                        } => {
                            invalid_argument_auditor.add_backtrace_part(
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                ),
                            );

                            return Ok(
                                InvalidArgumentResult::InvalidArgumentAuditor {
                                    invalid_argument_auditor,
                                },
                            );
                        }
                    };

                    let asset_hash_set_for_creating = match Self::check_and_transform_asset_registry_to_hash_set(asset_registry_for_creating.as_slice()) {
                        InvalidArgumentResult::Ok {
                            subject,
                        } => subject,
                        InvalidArgumentResult::InvalidArgumentAuditor {
                            mut invalid_argument_auditor,
                        } => {
                            invalid_argument_auditor.add_backtrace_part(
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                ),
                            );

                            return Ok(
                                InvalidArgumentResult::InvalidArgumentAuditor {
                                    invalid_argument_auditor,
                                },
                            );
                        }
                    };

                    let asset_registry_for_deleting = match Self::check_and_transform_asset_registry_to_vec(incoming_.asset_registry_for_deleting) {
                        InvalidArgumentResult::Ok {
                            subject,
                        } => subject,
                        InvalidArgumentResult::InvalidArgumentAuditor {
                            mut invalid_argument_auditor,
                        } => {
                            invalid_argument_auditor.add_backtrace_part(
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                ),
                            );

                            return Ok(
                                InvalidArgumentResult::InvalidArgumentAuditor {
                                    invalid_argument_auditor,
                                },
                            );
                        }
                    };

                    if let InvalidArgumentResult::InvalidArgumentAuditor {
                        mut invalid_argument_auditor,
                    } = Self::check_and_transform_asset_registry_to_hash_set(asset_registry_for_deleting.as_slice())
                    {
                        invalid_argument_auditor.add_backtrace_part(
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        );

                        return Ok(
                            InvalidArgumentResult::InvalidArgumentAuditor {
                                invalid_argument_auditor,
                            },
                        );
                    }

                    let subportfolio_asset_1_registry = match ClickhouseRepository::<SubportfolioAsset_1>::get_all(
                        &clickhouse_client,
                        &by_3,
                    )
                    .await
                    {
                        Ok(subportfolio_asset_1_registry_) => subportfolio_asset_1_registry_,
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

                    let subportfolio_asset_1_registry_length = subportfolio_asset_1_registry.len();

                    if asset_registry_for_deleting_length > subportfolio_asset_1_registry_length {
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

                    if (subportfolio_asset_1_registry_length + asset_registry_for_creating_length - asset_registry_for_deleting_length)
                        > SubportfolioAsset::MAXIMUM_QUANTITY_PER_SUBPORTFOLIO
                    {
                        return Ok(
                            InvalidArgumentResult::Ok {
                                subject: UnifiedReport::precedent(Precedent::SubportfolioAsset_MaximumQuantityPerSubportfolio),
                            },
                        );
                    }

                    let mut existed_asset_hash_set = HashSet::<(
                        &'_ Exchange_Id,
                        Wallet_Id,
                        &'_ Asset_Network,
                        Asset_ChainId,
                        &'_ Asset_Id,
                    )>::new();

                    '_a: for subportfolio_asset_1 in subportfolio_asset_1_registry.iter() {
                        existed_asset_hash_set.insert(
                            (
                                &subportfolio_asset_1.exchange_id,
                                subportfolio_asset_1.wallet_id,
                                &subportfolio_asset_1.asset_network,
                                subportfolio_asset_1.asset_chain_id,
                                &subportfolio_asset_1.asset_id,
                            ),
                        );
                    }

                    let mut subportfolio_asset_registry___: Vec<SubportfolioAsset> = vec![];

                    let mut cross_asset_registry: Vec<Asset> = vec![];

                    let mut not_existed_asset_registry: Vec<Asset> = vec![];

                    let subportfolio_asset_updated_at = match Resolver::<UTCDateTime>::get_now_() {
                        Ok(subportfolio_asset_created_at_) => subportfolio_asset_created_at_,
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

                    '_a: for asset in asset_registry_for_deleting.into_iter() {
                        if asset_hash_set_for_creating.contains(
                            &(
                                &asset.exchange_id,
                                asset.wallet_id,
                                &asset.asset_network,
                                asset.asset_chain_id,
                                &asset.asset_id,
                            ),
                        ) {
                            cross_asset_registry.push(
                                Asset {
                                    exchange_id: Resolver::<DefaultValue>::to_option(asset.exchange_id),
                                    exchange_name: Resolver::<DefaultValue>::to_option(asset.exchange_name),
                                    wallet_id: Resolver::<DefaultValue>::to_option(asset.wallet_id),
                                    wallet_address: Resolver::<DefaultValue>::to_option(asset.wallet_address),
                                    wallet_label: Resolver::<DefaultValue>::to_option(asset.wallet_label),
                                    asset_network: Resolver::<DefaultValue>::to_option(asset.asset_network),
                                    asset_chain_id: Resolver::<DefaultValue>::to_option(asset.asset_chain_id),
                                    asset_id: asset.asset_id,
                                },
                            );
                        } else {
                            if existed_asset_hash_set.contains(
                                &(
                                    &asset.exchange_id,
                                    asset.wallet_id,
                                    &asset.asset_network,
                                    asset.asset_chain_id,
                                    &asset.asset_id,
                                ),
                            ) {
                                subportfolio_asset_registry___.push(
                                    SubportfolioAsset {
                                        user_id: User_Id(user_.id),
                                        subportfolio_id: incoming_.subportfolio_id.clone(),
                                        exchange_id: asset.exchange_id,
                                        exchange_name: asset.exchange_name,
                                        wallet_id: asset.wallet_id,
                                        wallet_address: asset.wallet_address,
                                        wallet_label: asset.wallet_label,
                                        asset_network: asset.asset_network,
                                        asset_chain_id: asset.asset_chain_id,
                                        asset_id: asset.asset_id,
                                        created_at: SubportfolioAsset_CreatedAt(subportfolio_asset_updated_at),
                                        updated_at: SubportfolioAsset_UpdatedAt(subportfolio_asset_updated_at),
                                        is_deleted: IsDeleted::create_deleted(),
                                    },
                                );
                            } else {
                                not_existed_asset_registry.push(
                                    Asset {
                                        exchange_id: Resolver::<DefaultValue>::to_option(asset.exchange_id),
                                        exchange_name: Resolver::<DefaultValue>::to_option(asset.exchange_name),
                                        wallet_id: Resolver::<DefaultValue>::to_option(asset.wallet_id),
                                        wallet_address: Resolver::<DefaultValue>::to_option(asset.wallet_address),
                                        wallet_label: Resolver::<DefaultValue>::to_option(asset.wallet_label),
                                        asset_network: Resolver::<DefaultValue>::to_option(asset.asset_network),
                                        asset_chain_id: Resolver::<DefaultValue>::to_option(asset.asset_chain_id),
                                        asset_id: asset.asset_id,
                                    },
                                );
                            }
                        }
                    }

                    if !cross_asset_registry.is_empty() {
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

                    if !not_existed_asset_registry.is_empty() {
                        return Ok(
                            InvalidArgumentResult::Ok {
                                subject: UnifiedReport::precedent(
                                    Precedent::SubportfolioAsset_DoesNotExist {
                                        asset_registry: not_existed_asset_registry,
                                    },
                                ),
                            },
                        );
                    }

                    let mut existed_asset_registry: Vec<Asset> = vec![];

                    '_a: for asset in asset_registry_for_creating.into_iter() {
                        if existed_asset_hash_set.contains(
                            &(
                                &asset.exchange_id,
                                asset.wallet_id,
                                &asset.asset_network,
                                asset.asset_chain_id,
                                &asset.asset_id,
                            ),
                        ) {
                            existed_asset_registry.push(
                                Asset {
                                    exchange_id: Resolver::<DefaultValue>::to_option(asset.exchange_id),
                                    exchange_name: Resolver::<DefaultValue>::to_option(asset.exchange_name),
                                    wallet_id: Resolver::<DefaultValue>::to_option(asset.wallet_id),
                                    wallet_address: Resolver::<DefaultValue>::to_option(asset.wallet_address),
                                    wallet_label: Resolver::<DefaultValue>::to_option(asset.wallet_label),
                                    asset_network: Resolver::<DefaultValue>::to_option(asset.asset_network),
                                    asset_chain_id: Resolver::<DefaultValue>::to_option(asset.asset_chain_id),
                                    asset_id: asset.asset_id,
                                },
                            );
                        } else {
                            subportfolio_asset_registry___.push(
                                SubportfolioAsset {
                                    user_id: User_Id(user_.id),
                                    subportfolio_id: incoming_.subportfolio_id.clone(),
                                    exchange_id: asset.exchange_id,
                                    exchange_name: asset.exchange_name,
                                    wallet_id: asset.wallet_id,
                                    wallet_address: asset.wallet_address,
                                    wallet_label: asset.wallet_label,
                                    asset_network: asset.asset_network,
                                    asset_chain_id: asset.asset_chain_id,
                                    asset_id: asset.asset_id,
                                    created_at: SubportfolioAsset_CreatedAt(subportfolio_asset_updated_at),
                                    updated_at: SubportfolioAsset_UpdatedAt(subportfolio_asset_updated_at),
                                    is_deleted: IsDeleted::create_not_deleted(),
                                },
                            );
                        }
                    }

                    if !existed_asset_registry.is_empty() {
                        return Ok(
                            InvalidArgumentResult::Ok {
                                subject: UnifiedReport::precedent(
                                    Precedent::SubportfolioAsset_AlreadyExist {
                                        asset_registry: existed_asset_registry,
                                    },
                                ),
                            },
                        );
                    }

                    subportfolio_asset_registry___
                };

                subportfolio_asset_registry__
            };

            subportfolio_asset_registry_
        };

        if !subportfolio_asset_registry.is_empty() {
            if let Err(mut error_auditor) = ClickhouseRepository::<SubportfolioAsset>::create(
                &clickhouse_client,
                subportfolio_asset_registry.as_slice(),
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
            }
        }

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::empty(),
            },
        );
    }

    fn check_and_transform_asset_registry_to_vec(asset_registry: Vec<Asset>) -> InvalidArgumentResult<Vec<Asset_>> {
        let mut asset_registry_: Vec<Asset_> = vec![];

        '_a: for asset in asset_registry {
            let asset_ = match (
                asset.exchange_id,
                asset.wallet_id,
            ) {
                (Some(exchange_id), Some(wallet_id)) => {
                    let exchange_name = match asset.exchange_name {
                        Some(exchange_name_) => exchange_name_,
                        None => {
                            return InvalidArgumentResult::InvalidArgumentAuditor {
                                invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                                    InvalidArgument::new(),
                                    BacktracePart::new(
                                        line!(),
                                        file!(),
                                    ),
                                ),
                            };
                        }
                    };

                    let wallet_address = match asset.wallet_address {
                        Some(wallet_address_) => wallet_address_,
                        None => {
                            return InvalidArgumentResult::InvalidArgumentAuditor {
                                invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                                    InvalidArgument::new(),
                                    BacktracePart::new(
                                        line!(),
                                        file!(),
                                    ),
                                ),
                            };
                        }
                    };

                    Asset_ {
                        exchange_id,
                        exchange_name,
                        wallet_id,
                        wallet_address,
                        wallet_label: Resolver::<DefaultValue>::from_option(asset.wallet_label),
                        asset_network: Resolver::<DefaultValue>::from_option(asset.asset_network),
                        asset_chain_id: Resolver::<DefaultValue>::from_option(asset.asset_chain_id),
                        asset_id: asset.asset_id,
                    }
                }
                (Some(exchange_id), None) => {
                    let exchange_name = match asset.exchange_name {
                        Some(exchange_name_) => exchange_name_,
                        None => {
                            return InvalidArgumentResult::InvalidArgumentAuditor {
                                invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                                    InvalidArgument::new(),
                                    BacktracePart::new(
                                        line!(),
                                        file!(),
                                    ),
                                ),
                            };
                        }
                    };

                    let wallet_address = match asset.wallet_address {
                        Some(_) => {
                            return InvalidArgumentResult::InvalidArgumentAuditor {
                                invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                                    InvalidArgument::new(),
                                    BacktracePart::new(
                                        line!(),
                                        file!(),
                                    ),
                                ),
                            };
                        }
                        None => Wallet_Address::default(),
                    };

                    let wallet_label = match asset.wallet_label {
                        Some(_) => {
                            return InvalidArgumentResult::InvalidArgumentAuditor {
                                invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                                    InvalidArgument::new(),
                                    BacktracePart::new(
                                        line!(),
                                        file!(),
                                    ),
                                ),
                            };
                        }
                        None => Wallet_Label::default(),
                    };

                    let asset_network = match asset.asset_network {
                        Some(_) => {
                            return InvalidArgumentResult::InvalidArgumentAuditor {
                                invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                                    InvalidArgument::new(),
                                    BacktracePart::new(
                                        line!(),
                                        file!(),
                                    ),
                                ),
                            };
                        }
                        None => Asset_Network::default(),
                    };

                    let asset_chain_id = match asset.asset_chain_id {
                        Some(_) => {
                            return InvalidArgumentResult::InvalidArgumentAuditor {
                                invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                                    InvalidArgument::new(),
                                    BacktracePart::new(
                                        line!(),
                                        file!(),
                                    ),
                                ),
                            };
                        }
                        None => Asset_ChainId::default(),
                    };

                    Asset_ {
                        exchange_id,
                        exchange_name,
                        wallet_id: Wallet_Id::default(),
                        wallet_address,
                        wallet_label,
                        asset_network,
                        asset_chain_id,
                        asset_id: asset.asset_id,
                    }
                }
                (None, Some(wallet_id)) => {
                    let exchange_name = match asset.exchange_name {
                        Some(_) => {
                            return InvalidArgumentResult::InvalidArgumentAuditor {
                                invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                                    InvalidArgument::new(),
                                    BacktracePart::new(
                                        line!(),
                                        file!(),
                                    ),
                                ),
                            };
                        }
                        None => Exchange_Name::default(),
                    };

                    let wallet_address = match asset.wallet_address {
                        Some(wallet_address_) => wallet_address_,
                        None => {
                            return InvalidArgumentResult::InvalidArgumentAuditor {
                                invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                                    InvalidArgument::new(),
                                    BacktracePart::new(
                                        line!(),
                                        file!(),
                                    ),
                                ),
                            };
                        }
                    };

                    let asset_network = match asset.asset_network {
                        Some(asset_network_) => asset_network_,
                        None => {
                            return InvalidArgumentResult::InvalidArgumentAuditor {
                                invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                                    InvalidArgument::new(),
                                    BacktracePart::new(
                                        line!(),
                                        file!(),
                                    ),
                                ),
                            };
                        }
                    };

                    Asset_ {
                        exchange_id: Exchange_Id::default(),
                        exchange_name,
                        wallet_id,
                        wallet_address,
                        wallet_label: Resolver::<DefaultValue>::from_option(asset.wallet_label),
                        asset_network,
                        asset_chain_id: Resolver::<DefaultValue>::from_option(asset.asset_chain_id),
                        asset_id: asset.asset_id,
                    }
                }
                (None, None) => {
                    return InvalidArgumentResult::InvalidArgumentAuditor {
                        invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                            InvalidArgument::new(),
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    };
                }
            };

            asset_registry_.push(asset_);
        }

        return InvalidArgumentResult::Ok {
            subject: asset_registry_,
        };
    }

    fn check_and_transform_asset_registry_to_hash_set<'a>(
        asset_registry: &'a [Asset_],
    ) -> InvalidArgumentResult<
        HashSet<(
            &'a Exchange_Id,
            Wallet_Id,
            &'a Asset_Network,
            Asset_ChainId,
            &'a Asset_Id,
        )>,
    > {
        let mut asset_hash_set = HashSet::<(
            &'_ Exchange_Id,
            Wallet_Id,
            &'_ Asset_Network,
            Asset_ChainId,
            &'_ Asset_Id,
        )>::new();

        '_a: for asset in asset_registry {
            if !asset_hash_set.insert(
                (
                    &asset.exchange_id,
                    asset.wallet_id,
                    &asset.asset_network,
                    asset.asset_chain_id,
                    &asset.asset_id,
                ),
            ) {
                return InvalidArgumentResult::InvalidArgumentAuditor {
                    invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                        InvalidArgument::new(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                };
            }
        }

        return InvalidArgumentResult::Ok {
            subject: asset_hash_set,
        };
    }
}

pub struct Asset_ {
    exchange_id: Exchange_Id,
    exchange_name: Exchange_Name,
    wallet_id: Wallet_Id,
    wallet_address: Wallet_Address,
    wallet_label: Wallet_Label,
    asset_network: Asset_Network,
    asset_chain_id: Asset_ChainId,
    asset_id: Asset_Id,
}

#[cfg(feature = "not_authorized_user")]
#[derive(Deserialize)]
pub struct Incoming {
    user: User,
    subportfolio_id: Subportfolio_Id,
    asset_registry_for_creating: Vec<Asset>,
    asset_registry_for_deleting: Vec<Asset>,
}

#[cfg(not(feature = "not_authorized_user"))]
#[derive(Deserialize)]
pub struct Incoming {
    access_token: AccessToken,
    subportfolio_id: Subportfolio_Id,
    asset_registry_for_creating: Vec<Asset>,
    asset_registry_for_deleting: Vec<Asset>,
}

#[derive(Serialize, Deserialize)]
pub struct Asset {
    exchange_id: Option<Exchange_Id>,
    exchange_name: Option<Exchange_Name>,
    wallet_id: Option<Wallet_Id>,
    wallet_address: Option<Wallet_Address>,
    wallet_label: Option<Wallet_Label>,
    asset_network: Option<Asset_Network>,
    asset_chain_id: Option<Asset_ChainId>,
    asset_id: Asset_Id,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::Subportfolio_DoesNotExist,
        CommonPrecedent::SubportfolioAsset_MaximumQuantityPerSubportfolio,
        CommonPrecedent::SubportfolioAsset_AlreadyExist {
            asset_registry: Vec<Asset>
        },
        CommonPrecedent::SubportfolioAsset_DoesNotExist {
            asset_registry: Vec<Asset>
        }
    }
);

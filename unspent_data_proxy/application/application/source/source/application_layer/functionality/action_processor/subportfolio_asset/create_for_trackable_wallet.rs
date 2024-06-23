pub use crate::infrastructure_layer::data::control_type::SubportfolioAsset___CreateForTrackableWallet;
use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
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
            control_type::{
                DefaultValue,
                ServerAccessToken,
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
                resolver::{
                    utc_date_time::UTCDateTime,
                    Resolver,
                },
                validator::Validator as Validator_,
            },
        },
    },
};
use clickhouse::Client;
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashSet;

impl ActionProcessor<SubportfolioAsset___CreateForTrackableWallet> {
    pub async fn process(incoming: Option<Incoming>, clickhouse_client: Client) -> Result<InvalidArgumentResult<UnifiedReport<Void, Void>>, Auditor<Error>> {
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

        if !Validator_::<ServerAccessToken>::is_valid(&incoming_.server_access_token) {
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
            user_id: incoming_.user_id,
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
                    subject: UnifiedReport::empty(),
                },
            );
        }

        if incoming_.asset_registry.len() > 0 {
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

            let mut subportfolio_asset_1_hash_set = HashSet::<(
                &'_ Wallet_Id,
                &'_ Asset_Network,
                &'_ Asset_ChainId,
                &'_ Asset_Id,
            )>::new();

            '_a: for subportfolio_asset_1 in subportfolio_asset_1_registry.iter() {
                subportfolio_asset_1_hash_set.insert(
                    (
                        &subportfolio_asset_1.wallet_id,
                        &subportfolio_asset_1.asset_network,
                        &subportfolio_asset_1.asset_chain_id,
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

            let mut subportfolio_asset_registry: Vec<SubportfolioAsset> = vec![];

            '_a: for asset in incoming_.asset_registry.into_iter() {
                let asset_network = Resolver::<DefaultValue>::from_option(asset.asset_network);

                let asset_chain_id = Resolver::<DefaultValue>::from_option(asset.asset_chain_id);

                if !subportfolio_asset_1_hash_set.contains(
                    &(
                        &asset.wallet_id,
                        &asset_network,
                        &asset_chain_id,
                        &asset.asset_id,
                    ),
                ) {
                    subportfolio_asset_registry.push(
                        SubportfolioAsset {
                            user_id: User_Id(incoming_.user_id),
                            subportfolio_id: incoming_.subportfolio_id.clone(),
                            exchange_id: Exchange_Id::default(),
                            exchange_name: Exchange_Name::default(),
                            wallet_id: asset.wallet_id,
                            wallet_address: asset.wallet_address,
                            wallet_label: Resolver::<DefaultValue>::from_option(asset.wallet_label),
                            asset_network,
                            asset_chain_id,
                            asset_id: asset.asset_id,
                            created_at: SubportfolioAsset_CreatedAt(subportfolio_asset_created_at),
                            updated_at: SubportfolioAsset_UpdatedAt(subportfolio_asset_created_at),
                            is_deleted: IsDeleted::create_not_deleted(),
                        },
                    );
                }
            }

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
        }

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::empty(),
            },
        );
    }
}

#[derive(Deserialize)]
pub struct Incoming {
    server_access_token: ServerAccessToken,
    user_id: i32,
    subportfolio_id: Subportfolio_Id,
    asset_registry: Vec<Asset>,
}

#[derive(Serialize, Deserialize)]
pub struct Asset {
    wallet_id: Wallet_Id,
    wallet_address: Wallet_Address,
    wallet_label: Option<Wallet_Label>,
    asset_network: Option<Asset_Network>,
    asset_chain_id: Option<Asset_ChainId>,
    asset_id: Asset_Id,
}

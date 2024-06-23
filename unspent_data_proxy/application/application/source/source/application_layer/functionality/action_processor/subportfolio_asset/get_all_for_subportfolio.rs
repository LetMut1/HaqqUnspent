#[cfg(not(feature = "not_authorized_user"))]
use crate::infrastructure_layer::data::control_type::AccessToken;
pub use crate::infrastructure_layer::data::control_type::SubportfolioAsset___GetAllForSubportfolio;
#[cfg(feature = "not_authorized_user")]
use crate::infrastructure_layer::functionality::service::resolver::access_token::User;
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
                Wallet_Address,
                Wallet_Id,
                Wallet_Label,
            },
            subportfolio::Subportfolio_Id,
            subportfolio_asset::{
                SubportfolioAsset_2,
                SubportfolioAsset_CreatedAt,
            },
        },
        functionality::service::validator::Validator,
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            control_type::DefaultValue,
            error::{
                Error,
                Other,
                Runtime,
            },
            invalid_argument::InvalidArgument,
            invalid_argument_result::InvalidArgumentResult,
            void::Void,
        },
        functionality::{
            repository::clickhouse::{
                by::By3,
                ClickhouseRepository,
            },
            service::resolver::Resolver,
        },
    },
};
use clickhouse::Client;
use serde::{
    Deserialize,
    Serialize,
};

impl ActionProcessor<SubportfolioAsset___GetAllForSubportfolio> {
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

        let mut subportfolio_asset_2_row_cursor = match ClickhouseRepository::<SubportfolioAsset_2>::get_all(
            &clickhouse_client,
            &By3 {
                user_id: user_.id,
                subportfolio_id: incoming_.subportfolio_id.0.as_str(),
            },
        ) {
            Ok(subportfolio_asset_2_row_cursor) => subportfolio_asset_2_row_cursor,
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

        'a: loop {
            match subportfolio_asset_2_row_cursor.next().await {
                Ok(subportfolio_asset_2) => {
                    match subportfolio_asset_2 {
                        Some(subportfolio_asset_2_) => {
                            subportfolio_asset_registry.push(
                                SubportfolioAsset {
                                    exchange_id: Resolver::<DefaultValue>::to_option(subportfolio_asset_2_.exchange_id),
                                    exchange_name: Resolver::<DefaultValue>::to_option(subportfolio_asset_2_.exchange_name),
                                    wallet_id: Resolver::<DefaultValue>::to_option(subportfolio_asset_2_.wallet_id),
                                    wallet_address: Resolver::<DefaultValue>::to_option(subportfolio_asset_2_.wallet_address),
                                    wallet_label: Resolver::<DefaultValue>::to_option(subportfolio_asset_2_.wallet_label),
                                    asset_network: Resolver::<DefaultValue>::to_option(subportfolio_asset_2_.asset_network),
                                    asset_chain_id: Resolver::<DefaultValue>::to_option(subportfolio_asset_2_.asset_chain_id),
                                    asset_id: subportfolio_asset_2_.asset_id,
                                    created_at: subportfolio_asset_2_.created_at,
                                },
                            );
                        }
                        None => {
                            break 'a;
                        }
                    }
                }
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
            }
        }

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::filled(
                    Outcoming {
                        subportfolio_asset_registry,
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
}

#[cfg(not(feature = "not_authorized_user"))]
#[derive(Deserialize)]
pub struct Incoming {
    access_token: AccessToken,
    subportfolio_id: Subportfolio_Id,
}

#[derive(Serialize)]
pub struct Outcoming {
    subportfolio_asset_registry: Vec<SubportfolioAsset>,
}

#[derive(Serialize, Deserialize)]
pub struct SubportfolioAsset {
    exchange_id: Option<Exchange_Id>,
    exchange_name: Option<Exchange_Name>,
    wallet_id: Option<Wallet_Id>,
    wallet_address: Option<Wallet_Address>,
    wallet_label: Option<Wallet_Label>,
    asset_network: Option<Asset_Network>,
    asset_chain_id: Option<Asset_ChainId>,
    asset_id: Asset_Id,
    created_at: SubportfolioAsset_CreatedAt,
}

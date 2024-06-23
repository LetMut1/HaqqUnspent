pub use crate::infrastructure_layer::data::control_type::BaseBalanceSnapshot___Create;
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
            base_balance_snapshot::{
                BaseBalanceSnapshot,
                BaseBalanceSnapshot_Amount,
                BaseBalanceSnapshot_CreatedAt,
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
            repository::clickhouse::ClickhouseRepository,
            service::{
                resolver::Resolver,
                validator::{
                    server_access_token::ServerAccessToken,
                    Validator as Validator_,
                },
            },
        },
    },
};
use clickhouse::Client;
use serde::Deserialize;

impl ActionProcessor<BaseBalanceSnapshot___Create> {
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

        let mut base_balance_snapshot_registry: Vec<BaseBalanceSnapshot> = vec![];

        'a: for base_balance_snapshot in incoming_.base_balance_snapshot_registry.into_iter() {
            let is_valid_base_balance_snapshot_amount = match Validator::<BaseBalanceSnapshot_Amount>::is_valid(&base_balance_snapshot.amount) {
                Ok(is_valid_base_balance_snapshot_amount_) => is_valid_base_balance_snapshot_amount_,
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

            if !is_valid_base_balance_snapshot_amount {
                continue 'a;
            }

            let base_balance_snapshot_ = match (
                base_balance_snapshot.exchange_id,
                base_balance_snapshot.wallet_id,
            ) {
                (Some(exchange_id), Some(wallet_id)) => {
                    let exchange_name = match base_balance_snapshot.exchange_name {
                        Some(exchange_name_) => exchange_name_,
                        None => {
                            continue 'a;
                        }
                    };

                    let wallet_address = match base_balance_snapshot.wallet_address {
                        Some(wallet_address_) => wallet_address_,
                        None => {
                            continue 'a;
                        }
                    };

                    BaseBalanceSnapshot {
                        user_id: base_balance_snapshot.user_id,
                        exchange_id,
                        exchange_name,
                        wallet_id,
                        wallet_address,
                        wallet_label: Resolver::<DefaultValue>::from_option(base_balance_snapshot.wallet_label),
                        asset_network: Resolver::<DefaultValue>::from_option(base_balance_snapshot.asset_network),
                        asset_chain_id: Resolver::<DefaultValue>::from_option(base_balance_snapshot.asset_chain_id),
                        asset_id: base_balance_snapshot.asset_id,
                        amount: base_balance_snapshot.amount,
                        created_at: base_balance_snapshot.created_at,
                    }
                }
                (Some(exchange_id), None) => {
                    let exchange_name = match base_balance_snapshot.exchange_name {
                        Some(exchange_name_) => exchange_name_,
                        None => {
                            continue 'a;
                        }
                    };

                    let wallet_address = match base_balance_snapshot.wallet_address {
                        Some(_) => {
                            continue 'a;
                        }
                        None => Wallet_Address::default(),
                    };

                    let wallet_label = match base_balance_snapshot.wallet_label {
                        Some(_) => {
                            continue 'a;
                        }
                        None => Wallet_Label::default(),
                    };

                    let asset_network = match base_balance_snapshot.asset_network {
                        Some(_) => {
                            continue 'a;
                        }
                        None => Asset_Network::default(),
                    };

                    let asset_chain_id = match base_balance_snapshot.asset_chain_id {
                        Some(_) => {
                            continue 'a;
                        }
                        None => Asset_ChainId::default(),
                    };

                    BaseBalanceSnapshot {
                        user_id: base_balance_snapshot.user_id,
                        exchange_id,
                        exchange_name,
                        wallet_id: Wallet_Id::default(),
                        wallet_address,
                        wallet_label,
                        asset_network,
                        asset_chain_id,
                        asset_id: base_balance_snapshot.asset_id,
                        amount: base_balance_snapshot.amount,
                        created_at: base_balance_snapshot.created_at,
                    }
                }
                (None, Some(wallet_id)) => {
                    let exchange_name = match base_balance_snapshot.exchange_name {
                        Some(_) => {
                            continue 'a;
                        }
                        None => Exchange_Name::default(),
                    };

                    let wallet_address = match base_balance_snapshot.wallet_address {
                        Some(wallet_address_) => wallet_address_,
                        None => {
                            continue 'a;
                        }
                    };

                    let asset_network = match base_balance_snapshot.asset_network {
                        Some(asset_network_) => asset_network_,
                        None => {
                            continue 'a;
                        }
                    };

                    BaseBalanceSnapshot {
                        user_id: base_balance_snapshot.user_id,
                        exchange_id: Exchange_Id::default(),
                        exchange_name,
                        wallet_id,
                        wallet_address,
                        wallet_label: Resolver::<DefaultValue>::from_option(base_balance_snapshot.wallet_label),
                        asset_network,
                        asset_chain_id: Resolver::<DefaultValue>::from_option(base_balance_snapshot.asset_chain_id),
                        asset_id: base_balance_snapshot.asset_id,
                        amount: base_balance_snapshot.amount,
                        created_at: base_balance_snapshot.created_at,
                    }
                }
                (None, None) => {
                    continue 'a;
                }
            };

            base_balance_snapshot_registry.push(base_balance_snapshot_)
        }

        if !base_balance_snapshot_registry.is_empty() {
            if let Err(mut error_auditor) = ClickhouseRepository::<BaseBalanceSnapshot>::create(
                &clickhouse_client,
                base_balance_snapshot_registry.as_slice(),
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
}

#[derive(Deserialize)]
pub struct Incoming {
    server_access_token: ServerAccessToken,
    base_balance_snapshot_registry: Vec<BaseBalanceSnapshot_>,
}

#[derive(Deserialize)]
pub struct BaseBalanceSnapshot_ {
    user_id: User_Id,
    exchange_id: Option<Exchange_Id>,
    exchange_name: Option<Exchange_Name>,
    wallet_id: Option<Wallet_Id>,
    wallet_address: Option<Wallet_Address>,
    wallet_label: Option<Wallet_Label>,
    asset_network: Option<Asset_Network>,
    asset_chain_id: Option<Asset_ChainId>,
    asset_id: Asset_Id,
    amount: BaseBalanceSnapshot_Amount,
    created_at: BaseBalanceSnapshot_CreatedAt,
}

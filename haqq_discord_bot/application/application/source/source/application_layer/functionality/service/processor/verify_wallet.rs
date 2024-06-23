use super::run_bot::SharedData;
use super::run_bot::UndeletedComponentInteractionResponse;
use crate::application_layer::functionality::command_processor::run_bot::PROCESS_CAN_NOT_BE_INTERRUPTED_UNTIL_COMPLETED_QUANTITY;
use crate::application_layer::functionality::service::processor::Processor;
use crate::domain_layer::data::entity::address_verification_data::AddressVerificationData;
use crate::domain_layer::data::entity::discord_user_role::DiscordUserRole;
use crate::domain_layer::data::entity::discord_user_role::DiscordUserRole_1;
use crate::domain_layer::data::entity::discord_user_role::DiscordUserRole_2;
use crate::domain_layer::data::entity::discord_user_role::DiscordUserRole_3;
use crate::domain_layer::data::entity::discord_user_role::DiscordUserRole_4;
use crate::domain_layer::data::entity::raffle::Raffle_1;
use crate::domain_layer::data::entity::raffle_participant::RaffleParticipant;
use crate::domain_layer::data::entity::verified_address_blacklist::VerifiedAddressBlacklist;
use crate::domain_layer::data::entity::verified_bech32_address::VerifiedBech32Address;
use crate::domain_layer::data::entity::verified_bech32_address::VerifiedBech32Address_1;
use crate::domain_layer::data::entity::verified_bech32_address::VerifiedBech32Address_2;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::control_type::Cosmos;
use crate::infrastructure_layer::data::control_type::TokioNonBlockingTask;
use crate::infrastructure_layer::data::control_type::UTCDateTime;
use crate::infrastructure_layer::data::control_type::EVM;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_3;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_4;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_5;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_6;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_7;
use crate::infrastructure_layer::functionality::repository::postgresql::update::Update_1;
use crate::infrastructure_layer::functionality::repository::postgresql::update::Update_2;
use crate::infrastructure_layer::functionality::repository::postgresql::update::Update_3;
use crate::infrastructure_layer::functionality::repository::postgresql::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::encoder::Encoder;
use crate::infrastructure_layer::functionality::service::http_request_resolver::evm::EvmTransactionRegistryByAddressResponse;
use crate::infrastructure_layer::functionality::service::http_request_resolver::HttpRequestResolver;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use crate::infrastructure_layer::functionality::service::spawner::Spawner;
use crate::ISLM_COIN_PRECISION;
use bech32::Bech32;
use ethers::types::U256;
use ethers::utils::format_units;
use serenity::all::ComponentInteraction;
use serenity::all::CreateAllowedMentions;
use serenity::all::CreateInteractionResponseFollowup;
use serenity::all::Http;
use serenity::all::MessageId;
use serenity::model::id::UserId;
use std::clone::Clone;
use std::str::FromStr;
use std::sync::Arc;
use tokio::time::sleep;
use tokio::time::Duration;

pub use crate::infrastructure_layer::data::control_type::VerifyWallet;

impl Processor<VerifyWallet> {
    pub async fn process(
        process_can_not_be_interrupted_until_completed_quantity_incremented: bool,
        address_verification_data_aggregate: AddressVerificationDataAggregate,
        http: Arc<Http>,
        component_interaction: ComponentInteraction,
        shared_data: Arc<SharedData>,
    ) -> Result<(), Auditor<Error>> {
        if !process_can_not_be_interrupted_until_completed_quantity_incremented {
            *PROCESS_CAN_NOT_BE_INTERRUPTED_UNTIL_COMPLETED_QUANTITY.lock().await += 1;
        }

        let address_verification_data_aggregate_ = Arc::new(address_verification_data_aggregate);

        let component_interaction_ = Arc::new(component_interaction);

        'a: loop {
            if let Err(mut error_auditor) = Self::process_1(
                address_verification_data_aggregate_.clone(),
                http.clone(),
                component_interaction_.clone(),
                shared_data.clone(),
            )
            .await
            {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                Logger::<Auditor<Error>>::log(&error_auditor);

                continue 'a;
            }

            break 'a;
        }

        {
            *PROCESS_CAN_NOT_BE_INTERRUPTED_UNTIL_COMPLETED_QUANTITY.lock().await -= 1;
        }

        return Ok(());
    }

    async fn process_1<'a>(
        address_verification_data_aggregate: Arc<AddressVerificationDataAggregate>,
        http: Arc<Http>,
        component_interaction: Arc<ComponentInteraction>,
        shared_data: Arc<SharedData>,
    ) -> Result<(), Auditor<Error>> {
        let address_verification_data_aggregate_ = address_verification_data_aggregate.as_ref();

        'a: loop {
            if Resolver::<UTCDateTime>::get_now() <= address_verification_data_aggregate_.address_verification_data.expired_at {
                sleep(Duration::from_secs(6)).await;

                let balance = match HttpRequestResolver::<Cosmos>::get_aislm_balance_by_address(
                    address_verification_data_aggregate_.address_verification_data.recipient_bech32_address.as_str(),
                )
                .await
                {
                    Ok(balance_) => balance_,
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

                if balance.balance.amount.as_str() != "0" {
                    let evm_transaction_registry_by_address_response =
                        match HttpRequestResolver::<EVM>::get_transaction_registry_by_address(address_verification_data_aggregate_.recipient_evm_address.as_str()).await {
                            Ok(evm_transaction_registry_by_address_response_) => evm_transaction_registry_by_address_response_,
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

                    match evm_transaction_registry_by_address_response {
                        EvmTransactionRegistryByAddressResponse::ItemRegistry {
                            item_registry,
                        } => {
                            if !item_registry.items.is_empty() {
                                '_b: for item in item_registry.items.into_iter() {
                                    let aislm_amount = match U256::from_dec_str(item.value.as_str()) {
                                        Ok(aislm_amount_) => aislm_amount_,
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

                                    let islm_amount = match format_units(
                                        aislm_amount,
                                        ISLM_COIN_PRECISION,
                                    ) {
                                        Ok(islm_amount_) => islm_amount_,
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

                                    if item.status.as_str() == "ok"
                                        && item.result.as_str() == "success"
                                        && item.tx_types.len() == 1
                                        && item.tx_types[0].as_str() == "coin_transfer"
                                        && (islm_amount
                                            .find(address_verification_data_aggregate_.address_verification_data_expected_token_quantity_human_readable.as_str())
                                            .is_some()
                                            || islm_amount
                                                .find(
                                                    address_verification_data_aggregate_
                                                        .address_verification_data_expected_inaccurate_token_quantity_human_readable
                                                        .as_str(),
                                                )
                                                .is_some())
                                    {
                                        let bech32_address_from = match Encoder::<Bech32>::encode(item.from.hash.as_str()) {
                                            Ok(bech32_address_from_) => bech32_address_from_,
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

                                        if let Err(mut error_auditor) = Self::process_2(
                                            address_verification_data_aggregate,
                                            http,
                                            component_interaction,
                                            shared_data,
                                            bech32_address_from,
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

                                        return Ok(());
                                    }
                                }
                            }
                        }
                        _ => {}
                    }

                    let cosmos_transaction_registry_by_address_response = match HttpRequestResolver::<Cosmos>::get_transaction_registry_by_transfer_recipient_address(
                        address_verification_data_aggregate_.address_verification_data.recipient_bech32_address.as_str(),
                    )
                    .await
                    {
                        Ok(cosmos_transaction_registry_by_address_response_) => cosmos_transaction_registry_by_address_response_,
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

                    if !cosmos_transaction_registry_by_address_response.txs.is_empty() {
                        '_b: for transaction in cosmos_transaction_registry_by_address_response.txs.into_iter() {
                            'c: for message in transaction.body.messages.into_iter() {
                                let type_value = match message.get("@type") {
                                    Some(type_value_) => type_value_,
                                    None => {
                                        return Err(
                                            Auditor::<Error>::new(
                                                Error::create_value_does_not_exist(),
                                                BacktracePart::new(
                                                    line!(),
                                                    file!(),
                                                ),
                                            ),
                                        );
                                    }
                                };

                                let r#type = match type_value.as_str() {
                                    Some(r#type_) => r#type_,
                                    None => {
                                        return Err(
                                            Auditor::<Error>::new(
                                                Error::create_invalid_value(),
                                                BacktracePart::new(
                                                    line!(),
                                                    file!(),
                                                ),
                                            ),
                                        );
                                    }
                                };

                                if r#type == "/cosmos.bank.v1beta1.MsgSend" {
                                    let from_address_value = match message.get("from_address") {
                                        Some(from_address_value_) => from_address_value_,
                                        None => {
                                            return Err(
                                                Auditor::<Error>::new(
                                                    Error::create_value_does_not_exist(),
                                                    BacktracePart::new(
                                                        line!(),
                                                        file!(),
                                                    ),
                                                ),
                                            );
                                        }
                                    };

                                    let from_address = match from_address_value.as_str() {
                                        Some(from_address_) => from_address_,
                                        None => {
                                            return Err(
                                                Auditor::<Error>::new(
                                                    Error::create_invalid_value(),
                                                    BacktracePart::new(
                                                        line!(),
                                                        file!(),
                                                    ),
                                                ),
                                            );
                                        }
                                    };

                                    let amount_value = match message.get("amount") {
                                        Some(amount_value_) => amount_value_,
                                        None => {
                                            return Err(
                                                Auditor::<Error>::new(
                                                    Error::create_value_does_not_exist(),
                                                    BacktracePart::new(
                                                        line!(),
                                                        file!(),
                                                    ),
                                                ),
                                            );
                                        }
                                    };

                                    let amount = match amount_value.as_array() {
                                        Some(amount_) => amount_,
                                        None => {
                                            return Err(
                                                Auditor::<Error>::new(
                                                    Error::create_invalid_value(),
                                                    BacktracePart::new(
                                                        line!(),
                                                        file!(),
                                                    ),
                                                ),
                                            );
                                        }
                                    };

                                    'd: for value in amount.into_iter() {
                                        let denom_value = match value.get("denom") {
                                            Some(denom_value_) => denom_value_,
                                            None => {
                                                return Err(
                                                    Auditor::<Error>::new(
                                                        Error::create_value_does_not_exist(),
                                                        BacktracePart::new(
                                                            line!(),
                                                            file!(),
                                                        ),
                                                    ),
                                                );
                                            }
                                        };

                                        let denom = match denom_value.as_str() {
                                            Some(denom_) => denom_,
                                            None => {
                                                return Err(
                                                    Auditor::<Error>::new(
                                                        Error::create_invalid_value(),
                                                        BacktracePart::new(
                                                            line!(),
                                                            file!(),
                                                        ),
                                                    ),
                                                );
                                            }
                                        };

                                        let amount_value_ = match value.get("amount") {
                                            Some(amount_value__) => amount_value__,
                                            None => {
                                                return Err(
                                                    Auditor::<Error>::new(
                                                        Error::create_value_does_not_exist(),
                                                        BacktracePart::new(
                                                            line!(),
                                                            file!(),
                                                        ),
                                                    ),
                                                );
                                            }
                                        };

                                        let amount_ = match amount_value_.as_str() {
                                            Some(amount__) => amount__,
                                            None => {
                                                return Err(
                                                    Auditor::<Error>::new(
                                                        Error::create_invalid_value(),
                                                        BacktracePart::new(
                                                            line!(),
                                                            file!(),
                                                        ),
                                                    ),
                                                );
                                            }
                                        };

                                        if denom == "aISLM" {
                                            let aislm_amount = match U256::from_dec_str(amount_) {
                                                Ok(aislm_amount_) => aislm_amount_,
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

                                            let islm_amount = match format_units(
                                                aislm_amount,
                                                ISLM_COIN_PRECISION,
                                            ) {
                                                Ok(islm_amount_) => islm_amount_,
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

                                            if islm_amount
                                                .find(address_verification_data_aggregate_.address_verification_data_expected_token_quantity_human_readable.as_str())
                                                .is_some()
                                                || islm_amount
                                                    .find(
                                                        address_verification_data_aggregate_
                                                            .address_verification_data_expected_inaccurate_token_quantity_human_readable
                                                            .as_str(),
                                                    )
                                                    .is_some()
                                            {
                                                if let Err(mut error_auditor) = Self::process_2(
                                                    address_verification_data_aggregate,
                                                    http,
                                                    component_interaction,
                                                    shared_data,
                                                    from_address.to_string(),
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

                                                return Ok(());
                                            }
                                        } else {
                                            break 'd;
                                        }
                                    }
                                } else {
                                    break 'c;
                                }
                            }
                        }
                    }
                } else {
                    continue 'a;
                }
            } else {
                break 'a;
            }
        }

        return Ok(());
    }

    async fn process_2(
        address_verification_data_aggregate: Arc<AddressVerificationDataAggregate>,
        http: Arc<Http>,
        component_interaction: Arc<ComponentInteraction>,
        shared_data: Arc<SharedData>,
        bech32_address_from: String,
    ) -> Result<(), Auditor<Error>> {
        let postgresql_pooled_connection = match shared_data.as_ref().postgresql_connection_pool.as_ref().get().await {
            Ok(postgresql_pooled_connection_) => postgresql_pooled_connection_,
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

        let client = &*postgresql_pooled_connection;

        let is_exist = match PostgresqlRepository::<VerifiedAddressBlacklist>::is_exist(
            client,
            &By_4 {
                verified_address_blacklist_bech32_address: bech32_address_from.as_str(),
            },
        )
        .await
        {
            Ok(is_exist) => is_exist,
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

        if is_exist {
            let message = format!(
                "The wallet {} is on the blacklist.",
                bech32_address_from.as_str()
            );

            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                Self::notify(
                    http.clone(),
                    shared_data.clone(),
                    component_interaction,
                    message,
                ),
            );

            return Ok(());
        }

        let existed_verified_bech32_address_2 = match PostgresqlRepository::<VerifiedBech32Address_2>::find(
            client,
            &By_5 {
                verified_bech32_address_value: bech32_address_from.as_str(),
            },
        )
        .await
        {
            Ok(existed_verified_bech32_address_2_) => existed_verified_bech32_address_2_,
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

        if let Some(existed_verified_bech32_address_2_) = existed_verified_bech32_address_2 {
            if existed_verified_bech32_address_2_.discord_user_id == component_interaction.as_ref().user.id.to_string() {
                let message = format!(
                    "The wallet {} is already verified.",
                    bech32_address_from.as_str()
                );

                Spawner::<TokioNonBlockingTask>::spawn_into_background(
                    Self::notify(
                        http.clone(),
                        shared_data.clone(),
                        component_interaction,
                        message,
                    ),
                );
            } else {
                let verified_bech32_address_registry = match PostgresqlRepository::<VerifiedBech32Address_1>::get_all_available(
                    client,
                    &By_3 {
                        verified_bech32_address_discord_user_id: existed_verified_bech32_address_2_.discord_user_id.as_str(),
                    },
                )
                .await
                {
                    Ok(verified_bech32_address_registry_) => verified_bech32_address_registry_,
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

                if verified_bech32_address_registry.len() == 1 {
                    let raffle_1 = match PostgresqlRepository::<Raffle_1>::find_in_progress(client).await {
                        Ok(raffle_1_) => raffle_1_,
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

                    todo!("check raffle status");
                    if let Some(raffle_1_) = raffle_1 {
                        if let Err(mut error_auditor) = PostgresqlRepository::<RaffleParticipant>::delete(
                            client,
                            &By_6 {
                                raffle_participant_raffle_id: raffle_1_.id,
                                raffle_participant_discord_user_id: existed_verified_bech32_address_2_.discord_user_id.as_str(),
                            },
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

                    let discord_user_id = match UserId::from_str(existed_verified_bech32_address_2_.discord_user_id.as_str()) {
                        Ok(discord_user_id_) => discord_user_id_,
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

                    let shared_data_ = shared_data.as_ref();

                    if let Err(error) = http
                        .as_ref()
                        .remove_member_role(
                            shared_data_.discord_guild_id,
                            discord_user_id,
                            shared_data_.discord_guild_wallet_verified_role_id,
                            None,
                        )
                        .await
                    {
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

                    if let Err(error) = http
                        .as_ref()
                        .remove_member_role(
                            shared_data_.discord_guild_id,
                            discord_user_id,
                            shared_data_.discord_guild_stakers_club_member_role_id,
                            None,
                        )
                        .await
                    {
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

                let verified_address_blacklist = VerifiedAddressBlacklist::new(
                    bech32_address_from,
                    Resolver::<UTCDateTime>::get_now(),
                );

                if let Err(mut error_auditor) = PostgresqlRepository::<VerifiedAddressBlacklist>::create(
                    client,
                    &verified_address_blacklist,
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

                let message = format!(
                    "The wallet {} has been added to the blacklist.",
                    verified_address_blacklist.bech32_address.as_str()
                );

                Spawner::<TokioNonBlockingTask>::spawn_into_background(
                    Self::notify(
                        http.clone(),
                        shared_data.clone(),
                        component_interaction,
                        message,
                    ),
                );
            }

            return Ok(());
        }

        let verified_bech32_address_ = VerifiedBech32Address::new(
            bech32_address_from,
            address_verification_data_aggregate.as_ref().address_verification_data.discord_user_id.clone(),
            Resolver::<UTCDateTime>::get_now(),
        );

        if let Err(mut error_auditor) = PostgresqlRepository::<VerifiedBech32Address>::create(
            client,
            &verified_bech32_address_,
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

        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            Self::add_roles_and_notify(
                http,
                component_interaction.clone(),
                shared_data.clone(),
                verified_bech32_address_.value,
            ),
        );

        return Ok(());
    }

    async fn add_roles_and_notify(
        http: Arc<Http>,
        component_interaction: Arc<ComponentInteraction>,
        shared_data: Arc<SharedData>,
        bech32_address: String,
    ) -> Result<(), Auditor<Error>> {
        'a: loop {
            if let Err(mut error_auditor) = Self::add_roles_and_notify_(
                http.clone(),
                component_interaction.clone(),
                shared_data.clone(),
                bech32_address.clone(),
            )
            .await
            {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                Logger::<Auditor<Error>>::log(&error_auditor);

                sleep(Duration::from_secs(5)).await;

                continue 'a;
            }

            break 'a;
        }

        return Ok(());
    }

    async fn add_roles_and_notify_(
        http: Arc<Http>,
        component_interaction: Arc<ComponentInteraction>,
        shared_data: Arc<SharedData>,
        bech32_address: String,
    ) -> Result<(), Auditor<Error>> {
        let component_interaction_ = component_interaction.as_ref();

        let user_ = &component_interaction_.user;

        let postgresql_pooled_connection = match shared_data.as_ref().postgresql_connection_pool.as_ref().get().await {
            Ok(postgresql_pooled_connection_) => postgresql_pooled_connection_,
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

        let client = &*postgresql_pooled_connection;

        let discord_user_id = user_.id.to_string();

        let discord_user_role_1 = match PostgresqlRepository::<DiscordUserRole_1>::find(
            client,
            &By_7 {
                discord_user_role_discord_user_id: discord_user_id.as_str(),
            },
        )
        .await
        {
            Ok(discord_user_role_1_) => discord_user_role_1_,
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

        match discord_user_role_1 {
            Some(discord_user_role_1_) => {
                let mut is_needed_to_add_role_wallet_verified = false;

                if !discord_user_role_1_.wallet_verified {
                    if let Err(mut error_auditor) = Self::add_role_wallet_verified(
                        http.clone(),
                        user_.id,
                        shared_data.clone(),
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

                    is_needed_to_add_role_wallet_verified = true;
                }

                let mut is_needed_to_add_role_stakers_club_member = false;

                if !discord_user_role_1_.stakers_club_member {
                    let is_role_stakers_club_member_added = match Self::try_add_role_stakers_club_member(
                        http.clone(),
                        user_.id,
                        shared_data.clone(),
                    )
                    .await
                    {
                        Ok(is_role_stakers_club_member_added_) => is_role_stakers_club_member_added_,
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

                    is_needed_to_add_role_stakers_club_member = is_role_stakers_club_member_added;
                }

                if is_needed_to_add_role_wallet_verified && is_needed_to_add_role_stakers_club_member {
                    if let Err(mut error_auditor) = PostgresqlRepository::<DiscordUserRole_2>::update(
                        client,
                        Update_1 {
                            discord_user_role_wallet_verified: true,
                            discord_user_role_stakers_club_member: true,
                            discord_user_role_updated_at: Resolver::<UTCDateTime>::get_now(),
                        },
                        &By_7 {
                            discord_user_role_discord_user_id: discord_user_id.as_str(),
                        },
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
                } else {
                    if is_needed_to_add_role_wallet_verified {
                        if let Err(mut error_auditor) = PostgresqlRepository::<DiscordUserRole_3>::update(
                            client,
                            Update_2 {
                                discord_user_role_wallet_verified: true,
                                discord_user_role_updated_at: Resolver::<UTCDateTime>::get_now(),
                            },
                            &By_7 {
                                discord_user_role_discord_user_id: discord_user_id.as_str(),
                            },
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

                    if is_needed_to_add_role_stakers_club_member {
                        if let Err(mut error_auditor) = PostgresqlRepository::<DiscordUserRole_4>::update(
                            client,
                            Update_3 {
                                discord_user_role_stakers_club_member: true,
                                discord_user_role_updated_at: Resolver::<UTCDateTime>::get_now(),
                            },
                            &By_7 {
                                discord_user_role_discord_user_id: discord_user_id.as_str(),
                            },
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
                }
            }
            None => {
                if let Err(mut error_auditor) = Self::add_role_wallet_verified(
                    http.clone(),
                    user_.id,
                    shared_data.clone(),
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

                let is_role_stakers_club_member_added = match Self::try_add_role_stakers_club_member(
                    http.clone(),
                    user_.id,
                    shared_data.clone(),
                )
                .await
                {
                    Ok(is_role_stakers_club_member_added_) => is_role_stakers_club_member_added_,
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

                let discord_user_role = DiscordUserRole::new(
                    discord_user_id,
                    true,
                    is_role_stakers_club_member_added,
                    Resolver::<UTCDateTime>::get_now(),
                );

                if let Err(mut error_auditor) = PostgresqlRepository::<DiscordUserRole>::create(
                    client,
                    &discord_user_role,
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
        }

        let message = format!(
            "The wallet {} was verified.",
            bech32_address.as_str()
        );

        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            Self::notify(
                http.clone(),
                shared_data.clone(),
                component_interaction,
                message,
            ),
        );

        return Ok(());
    }

    async fn add_role_wallet_verified(
        http: Arc<Http>,
        user_id: UserId,
        shared_data: Arc<SharedData>,
    ) -> Result<(), Auditor<Error>> {
        let shared_data_ = shared_data.as_ref();

        if let Err(error) = http
            .as_ref()
            .add_member_role(
                shared_data_.discord_guild_id,
                user_id,
                shared_data_.discord_guild_wallet_verified_role_id,
                None,
            )
            .await
        {
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

        return Ok(());
    }

    async fn try_add_role_stakers_club_member(
        http: Arc<Http>,
        user_id: UserId,
        shared_data: Arc<SharedData>,
    ) -> Result<bool, Auditor<Error>> {
        let shared_data_ = shared_data.as_ref();

        let postgresql_pooled_connection = match shared_data.as_ref().postgresql_connection_pool.as_ref().get().await {
            Ok(postgresql_pooled_connection_) => postgresql_pooled_connection_,
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

        let verified_bech32_address_1_registry = match PostgresqlRepository::<VerifiedBech32Address_1>::get_all_available(
            &*postgresql_pooled_connection,
            &By_3 {
                verified_bech32_address_discord_user_id: user_id.to_string().as_str(),
            },
        )
        .await
        {
            Ok(verified_bech32_address_1_registry_) => verified_bech32_address_1_registry_,
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

        let mut aislm_stake_for_all_addresses_quantity = U256::zero();

        '_a: for verified_bech32_address_1 in verified_bech32_address_1_registry.into_iter() {
            let stake_quantity_by_address_response = match HttpRequestResolver::<Cosmos>::get_aislm_stake_quantity_by_address(verified_bech32_address_1.value.as_str()).await {
                Ok(stake_quantity_by_address_response_) => stake_quantity_by_address_response_,
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

            '_b: for delegation_response in stake_quantity_by_address_response.delegation_responses.into_iter() {
                let aislm_stake_quantity = match U256::from_dec_str(delegation_response.balance.amount.as_str()) {
                    Ok(aislm_stake_quantity_) => aislm_stake_quantity_,
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

                aislm_stake_for_all_addresses_quantity += aislm_stake_quantity;
            }
        }

        let mut is_role_added = false;

        if aislm_stake_for_all_addresses_quantity >= shared_data_.stake_treshold_quantity_for_stakers_club_role {
            if let Err(error) = http
                .as_ref()
                .add_member_role(
                    shared_data_.discord_guild_id,
                    user_id,
                    shared_data_.discord_guild_stakers_club_member_role_id,
                    None,
                )
                .await
            {
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

            is_role_added = true;
        }

        return Ok(is_role_added);
    }

    async fn notify(
        http: Arc<Http>,
        shared_data: Arc<SharedData>,
        component_interaction: Arc<ComponentInteraction>,
        message: String,
    ) -> Result<(), Auditor<Error>> {
        let mut is_needed_to_delete_interaction = true;

        let mut followup_interaction_message_id: Option<MessageId> = None;

        let mut counter = 0 as usize;

        'a: loop {
            if let Err(mut error_auditor) = Self::notify_(
                http.clone(),
                shared_data.clone(),
                component_interaction.clone(),
                message.clone(),
                &mut is_needed_to_delete_interaction,
                &mut followup_interaction_message_id,
            )
            .await
            {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                Logger::<Auditor<Error>>::log(&error_auditor);

                if counter
                    < shared_data
                        .as_ref()
                        .environment_configuration
                        .as_ref()
                        .noncontext_parameters
                        .algorithm_repetition_in_error_case_quantity
                {
                    sleep(Duration::from_secs(3)).await;

                    counter += 1;

                    continue 'a;
                } else {
                    break 'a;
                }
            }

            break 'a;
        }

        return Ok(());
    }

    async fn notify_<'a>(
        http: Arc<Http>,
        shared_data: Arc<SharedData>,
        component_interaction: Arc<ComponentInteraction>,
        message: String,
        is_needed_to_delete_interaction: &'a mut bool,
        followup_interaction_message_id: &'a mut Option<MessageId>,
    ) -> Result<(), Auditor<Error>> {
        let shared_data_ = shared_data.as_ref();

        let component_interaction_ = component_interaction.as_ref();

        if *is_needed_to_delete_interaction {
            let mut mutex_guard = shared_data_.wallet_verification_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

            if let Some(undeleted_component_interaction_response) = (*mutex_guard).get(&component_interaction.user.id) {
                match *(undeleted_component_interaction_response.as_ref()) {
                    UndeletedComponentInteractionResponse::InteractionResponse {
                        component_interaction: ref component_interaction_,
                    } => {
                        if let Err(error) = component_interaction_.delete_response(http.as_ref()).await {
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
                    UndeletedComponentInteractionResponse::InteractionResponseFollowup {
                        component_interaction: ref component_interaction_,
                        ref message_id,
                    } => {
                        if let Err(error) = component_interaction_
                            .delete_followup(
                                http.as_ref(),
                                *message_id,
                            )
                            .await
                        {
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

                let _ = (*mutex_guard).remove(&component_interaction.user.id);

                *is_needed_to_delete_interaction = false;
            }
        }

        let followup_interaction_message_id_ = match followup_interaction_message_id {
            Some(ref followup_interaction_message_id__) => *followup_interaction_message_id__,
            None => {
                let create_allowed_mentions = CreateAllowedMentions::new().users([component_interaction.user.id]);

                let create_component_interaction_followup = CreateInteractionResponseFollowup::new().content(message).allowed_mentions(create_allowed_mentions).ephemeral(true);

                let followup_interaction_message_id__ = match component_interaction_
                    .create_followup(
                        http.as_ref(),
                        create_component_interaction_followup,
                    )
                    .await
                {
                    Ok(followup_interaction_message_id___) => followup_interaction_message_id___.id,
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

                *followup_interaction_message_id = Some(followup_interaction_message_id__);

                followup_interaction_message_id__
            }
        };

        {
            let mut mutex_guard = shared_data_.wallet_verification_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

            let _ = (*mutex_guard).insert(
                component_interaction.user.id,
                Arc::new(
                    UndeletedComponentInteractionResponse::InteractionResponseFollowup {
                        component_interaction: component_interaction_.clone(),
                        message_id: followup_interaction_message_id_,
                    },
                ),
            );
        }

        return Ok(());
    }
}

pub struct AddressVerificationDataAggregate {
    pub address_verification_data: AddressVerificationData,
    pub recipient_evm_address: String,
    pub address_verification_data_expected_token_quantity_human_readable: String,
    pub address_verification_data_expected_inaccurate_token_quantity_human_readable: String,
}

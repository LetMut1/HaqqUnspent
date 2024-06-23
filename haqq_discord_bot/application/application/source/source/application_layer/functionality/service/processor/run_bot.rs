use bb8_postgres::PostgresConnectionManager;
use bb8::Pool;
use bech32::Bech32;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::k256::Secp256k1;
use ethers::signers::Wallet;
use crate::application_layer::functionality::command_processor::run_bot::IS_SHUTDOWN_SIGNAL_RECEIVED;
use crate::application_layer::functionality::command_processor::run_bot::PROCESS_CAN_NOT_BE_INTERRUPTED_UNTIL_COMPLETED_QUANTITY;
use crate::application_layer::functionality::service::processor::Processor;
use crate::application_layer::functionality::service::processor::update_discord_roles::UpdateDiscordRoles;
use crate::domain_layer::data::entity::address_verification_data::AddressVerificationData_ExpiredAt;
use crate::domain_layer::data::entity::address_verification_data::AddressVerificationData;
use crate::domain_layer::data::entity::aislm_stake::AislmStake_1;
use crate::domain_layer::data::entity::aislm_stake::AislmStake_2;
use crate::domain_layer::data::entity::discord_user_role::DiscordUserRole_1;
use crate::domain_layer::data::entity::recipient_hd_wallet::RecipientHdWallet_1;
use crate::domain_layer::data::entity::recipient_hd_wallet::RecipientHdWallet;
use crate::domain_layer::data::entity::raffle::Raffle_4;
use crate::domain_layer::data::entity::raffle_participant::RaffleParticipant;
use crate::domain_layer::data::entity::raffle::Raffle_1;
use crate::domain_layer::data::entity::raffle::Raffle_3;
use crate::domain_layer::data::entity::raffle::Raffle_Seed;
use crate::domain_layer::data::entity::raffle::Raffle_Status;
use crate::domain_layer::data::entity::raffle::Raffle;
use crate::domain_layer::data::entity::sender_hd_wallet::SenderHdWallet;
use crate::domain_layer::data::entity::sender_hd_wallet::SenderHdWallet_1;
use crate::domain_layer::data::entity::verified_address_blacklist::VerifiedAddressBlacklist;
use crate::domain_layer::data::entity::verified_bech32_address::VerifiedBech32Address_1;
use crate::domain_layer::functionality::service::creator::Creator;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::control_type::Common;
use crate::infrastructure_layer::data::control_type::Cosmos;
use crate::infrastructure_layer::data::control_type::DiscordCompositeCustomId;
use crate::infrastructure_layer::data::control_type::TokioNonBlockingTask;
use crate::infrastructure_layer::data::control_type::UTCDateTime;
use crate::infrastructure_layer::data::control_type::VerifyWallet;
use crate::infrastructure_layer::data::control_type::CompleteRaffle;
use crate::infrastructure_layer::data::control_type::UpdateRaffle;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_1;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_3;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_4;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_6;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_7;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_8;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_9;
use crate::infrastructure_layer::functionality::repository::postgresql::insert::Insert_1;
use crate::infrastructure_layer::functionality::repository::postgresql::insert::Insert_2;
use crate::infrastructure_layer::functionality::repository::postgresql::insert::Insert_4;
use crate::infrastructure_layer::functionality::repository::postgresql::update::Update_5;
use crate::infrastructure_layer::functionality::repository::postgresql::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::creator::postgresql_connection_pool_no_tls::PostgresqlConnectionPoolNoTls;
use crate::infrastructure_layer::functionality::service::encoder::Encoder;
use crate::infrastructure_layer::functionality::service::http_request_resolver::HttpRequestResolver;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use crate::infrastructure_layer::functionality::service::spawner::Spawner;
use crate::ISLM_COIN_PRECISION;
use ethers::core::rand;
use ethers::signers::coins_bip39::English;
use ethers::signers::coins_bip39::Mnemonic;
use ethers::signers::MnemonicBuilder;
use ethers::signers::Signer;
use ethers::types::U256;
use ethers::utils::format_units;
use std::convert::TryFrom;
use ethers::utils::hex::ToHexExt;
use qrcode::QrCode;
use rand::Rng;
use rand::thread_rng;
use serenity::all::ChannelId;
use serenity::all::ClientBuilder;
use serenity::all::CommandInteraction;
use serenity::all::ComponentInteraction;
use serenity::all::CreateButton;
use serenity::all::EditRole;
use serenity::all::Http;
use serenity::all::MessageId;
use serenity::all::RoleId;
use serenity::all::UserId;
use serenity::async_trait;
use serenity::builder::CreateAttachment;
use serenity::builder::CreateChannel;
use serenity::builder::CreateCommand;
use serenity::builder::CreateCommandOption;
use serenity::builder::CreateEmbed;
use serenity::builder::CreateInteractionResponse;
use serenity::builder::CreateInteractionResponseMessage;
use serenity::builder::CreateMessage;
use serenity::client::Context;
use serenity::client::EventHandler;
use serenity::model::application::CommandOptionType;
use serenity::model::application::Interaction;
use serenity::model::application::ResolvedValue;
use serenity::model::channel::ChannelType;
use serenity::model::channel::Message;
use serenity::model::channel::PermissionOverwrite;
use serenity::model::channel::PermissionOverwriteType;
use serenity::model::gateway::GatewayIntents;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::permissions::Permissions;
use serenity::prelude::TypeMapKey;
use std::clone::Clone;
use std::collections::HashMap;
use std::sync::Arc;
use crate::infrastructure_layer::data::control_type::ServeRaffle;
use crate::infrastructure_layer::data::control_type::CancelRaffle;
use super::verify_wallet::AddressVerificationDataAggregate;
use tokio_postgres::NoTls;
use tokio::sync::Mutex;
use tokio::time::Duration;
use tokio::time::sleep;

pub use crate::infrastructure_layer::data::control_type::RunBot;

impl Processor<RunBot> {
    const DISCORD_GUILD_ROLE_WALLET_VERIFIED: &'static str = "Wallet Verified";

    const DISCORD_GUILD_ROLE_STAKERS_CLUB_MEMBER: &'static str = "Stakers Club Member";

    const DISCORD_GUILD_CHANNEL_WALLET_VERIFICATION_NAME: &'static str = "wallet-verification";

    const DISCORD_GUILD_CHANNEL_STAKERS_CLUB_NAME: &'static str = "stakers-club";

    const CUSTOM_ID_1: &'static str = "1";

    const CUSTOM_ID_2: &'static str = "2";

    pub const CUSTOM_ID_3: &'static str = "3";

    const COMMAND_REMOVE_FROM_BLACKLIST: &'static str = "remove_from_blacklist";

    const COMMAND_RAFFLE_STATISTIC: &'static str = "raffle_statistic";

    const COMMAND_INITIATE_RAFFLE: &'static str = "initiate_raffle";

    const COMMAND_CANCEL_RAFFLE: &'static str = "cancel_raffle";

    const COMMAND_COMPLETE_RAFFLE: &'static str = "complete_raffle";

    const COMMAND_UPDATE_RAFFLE: &'static str = "update_raffle";

    const MESSAGE_RAFFLE_CHANCE: &'static str = "!raffle_chance";

    const MESSAGE_MY_STAKE: &'static str = "!my_stake";

    const RESPONSE_MESSAGE_FOR_SHUTDOWN_PROCESS: &'static str = "The bot is preparing to shut down for a while for maintenance. Please try again later.";

    pub async fn process(
        environment_configuration: Arc<EnvironmentConfiguration>,
        postgresql_connection_pool: Arc<PostgresqlConnectionPoolNoTls>,
    ) -> Result<(), Auditor<Error>> {
        let mut is_needed_to_create_verify_button = false;

        'a: loop {
            if let Err(mut error_auditor) = Self::process_1(
                environment_configuration.clone(),
                postgresql_connection_pool.clone(),
                &mut is_needed_to_create_verify_button,
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

    async fn process_1<'a>(
        environment_configuration: Arc<EnvironmentConfiguration>,
        postgresql_connection_pool: Arc<Pool<PostgresConnectionManager<NoTls>>>,
        is_needed_to_create_verify_button: &'a mut bool,
    ) -> Result<(), Auditor<Error>> {
        let environment_configuration_ = environment_configuration.as_ref();

        let bot_role_id = RoleId::new(environment_configuration_.remote_service.discord.application.bot.role_id);

        let stake_treshold_quantity_for_stakers_club_role =
            match U256::from_dec_str(environment_configuration_.noncontext_parameters.aislm_stake_streshold_quantity_for_stakers_club_role.as_str()) {
                Ok(streshold_stake_quantity_) => streshold_stake_quantity_,
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

        let (
            recipient_hd_wallet_id,
            sender_hd_wallet_id,
            raffle
        ) = {
            let postgresql_pooled_connection = match postgresql_connection_pool.as_ref().get().await {
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

            let recipient_hd_wallet_1 = match PostgresqlRepository::<RecipientHdWallet_1>::find(client).await {
                Ok(recipient_hd_wallet_1_) => recipient_hd_wallet_1_,
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

            let recipient_hd_wallet_id_ = match recipient_hd_wallet_1 {
                Some(recipient_hd_wallet_1__) => recipient_hd_wallet_1__.id,
                None => {
                    let recipient_hd_wallet_mnemonic_phrase = Mnemonic::<English>::new(&mut thread_rng()).to_phrase();

                    let recipient_hd_wallet = match PostgresqlRepository::<RecipientHdWallet>::create(
                        client,
                        Insert_1 {
                            recipient_hd_wallet_mnemonic_phrase,
                            recipient_hd_wallet_mnemonic_derevation_path_index: 0,
                            recipient_hd_wallet_created_at: Resolver::<UTCDateTime>::get_now(),
                        },
                    )
                    .await
                    {
                        Ok(recipient_hd_wallet_) => recipient_hd_wallet_,
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

                    recipient_hd_wallet.id
                }
            };

            let sender_hd_wallet_1 = match PostgresqlRepository::<SenderHdWallet_1>::find(client).await {
                Ok(sender_hd_wallet_1_) => sender_hd_wallet_1_,
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

            let sender_hd_wallet_id_ = match sender_hd_wallet_1 {
                Some(sender_hd_wallet_1__) => sender_hd_wallet_1__.id,
                None => {
                    let sender_hd_wallet_mnemonic_phrase = Mnemonic::<English>::new(&mut thread_rng()).to_phrase();

                    let sender_hd_wallet = match PostgresqlRepository::<SenderHdWallet>::create(
                        client,
                        Insert_4 {
                            sender_hd_wallet_mnemonic_phrase,
                            sender_hd_wallet_mnemonic_derevation_path_index: 0,
                            sender_hd_wallet_created_at: Resolver::<UTCDateTime>::get_now(),
                        }
                    )
                    .await
                    {
                        Ok(sender_hd_wallet_) => sender_hd_wallet_,
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

                    sender_hd_wallet.id
                }
            };

            let raffle_ = match PostgresqlRepository::<Raffle>::find_in_progress(client).await {
                Ok(raffle__) => raffle__,
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

            (
                recipient_hd_wallet_id_,
                sender_hd_wallet_id_,
                raffle_,
            )
        };

        let guild_id = GuildId::new(environment_configuration_.remote_service.discord.guild.id);

        let http = Resolver::<Http>::create(environment_configuration_);

        let mut existing_guild_role_hash_map = match guild_id.roles(&http).await {
            Ok(existed_guild_role_hash_map_) => existed_guild_role_hash_map_,
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

        let _ = existing_guild_role_hash_map.remove(&bot_role_id);

        let mut wallet_verified_role_id: Option<RoleId> = None;

        let mut stakers_club_member_role_id: Option<RoleId> = None;

        '_a: for (role_id, role) in existing_guild_role_hash_map.iter() {
            if role.name.as_str() == Self::DISCORD_GUILD_ROLE_WALLET_VERIFIED {
                wallet_verified_role_id = Some(*role_id);
            }

            if role.name.as_str() == Self::DISCORD_GUILD_ROLE_STAKERS_CLUB_MEMBER {
                stakers_club_member_role_id = Some(*role_id);
            }
        }

        let wallet_verified_role_id_ = match wallet_verified_role_id {
            Some(wallet_verified_role_id__) => wallet_verified_role_id__,
            None => {
                let edit_role = EditRole::new()
                    .name(Self::DISCORD_GUILD_ROLE_WALLET_VERIFIED)
                    .hoist(false)
                    .mentionable(true)
                    .permissions(Permissions::empty());

                let wallet_verified_role = match guild_id
                    .create_role(
                        &http, edit_role,
                    )
                    .await
                {
                    Ok(wallet_verified_role_) => wallet_verified_role_,
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

                let wallet_verified_role_id__ = wallet_verified_role.id;

                existing_guild_role_hash_map.insert(
                    wallet_verified_role_id__,
                    wallet_verified_role,
                );

                wallet_verified_role_id__
            }
        };

        let stakers_club_member_role_id_ = match stakers_club_member_role_id {
            Some(stakers_club_member_role_id__) => stakers_club_member_role_id__,
            None => {
                let edit_role = EditRole::new()
                    .name(Self::DISCORD_GUILD_ROLE_STAKERS_CLUB_MEMBER)
                    .hoist(false)
                    .mentionable(true)
                    .permissions(Permissions::empty());

                let stakers_club_role = match guild_id
                    .create_role(
                        &http, edit_role,
                    )
                    .await
                {
                    Ok(stakers_club_role_) => stakers_club_role_,
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

                let stakers_club_role_id__ = stakers_club_role.id;

                existing_guild_role_hash_map.insert(
                    stakers_club_role_id__,
                    stakers_club_role,
                );

                stakers_club_role_id__
            }
        };

        let existed_channel_hash_map = match guild_id.channels(&http).await {
            Ok(existed_channel_hash_map_) => existed_channel_hash_map_,
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

        let mut wallet_verification_channel_id: Option<ChannelId> = None;

        let mut stakers_club_channel_id: Option<ChannelId> = None;

        '_a: for (channel_id, guild_channel) in existed_channel_hash_map.into_iter() {
            if guild_channel.name.as_str() == Self::DISCORD_GUILD_CHANNEL_WALLET_VERIFICATION_NAME {
                wallet_verification_channel_id = Some(channel_id);
            }

            if guild_channel.name.as_str() == Self::DISCORD_GUILD_CHANNEL_STAKERS_CLUB_NAME {
                stakers_club_channel_id = Some(channel_id);
            }
        }

        let wallet_verification_channel_id_ = match wallet_verification_channel_id {
            Some(wallet_verification_channel_id__) => wallet_verification_channel_id__,
            None => {
                let mut wallet_verification_permission_overwrite_registry: Vec<PermissionOverwrite> = vec![];

                '_a: for (role_id, _) in existing_guild_role_hash_map.iter() {
                    wallet_verification_permission_overwrite_registry.push(
                        PermissionOverwrite {
                            allow: Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY,
                            deny: Permissions::all(),
                            kind: PermissionOverwriteType::Role(*role_id),
                        },
                    )
                }

                let wallet_verification_create_channel = CreateChannel::new(Self::DISCORD_GUILD_CHANNEL_WALLET_VERIFICATION_NAME)
                    .kind(ChannelType::Text)
                    .permissions(wallet_verification_permission_overwrite_registry);

                let wallet_verification_guild_channel = match guild_id
                    .create_channel(
                        &http,
                        wallet_verification_create_channel,
                    )
                    .await
                {
                    Ok(wallet_verification_guild_channel_) => wallet_verification_guild_channel_,
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

                *is_needed_to_create_verify_button = true;

                wallet_verification_guild_channel.id
            }
        };

        if *is_needed_to_create_verify_button {
            let create_button = CreateButton::new(Self::CUSTOM_ID_1).label("Verify");

            let create_message = CreateMessage::new().button(create_button);

            if let Err(error) = wallet_verification_channel_id_
                .send_message(
                    &http,
                    create_message,
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

            *is_needed_to_create_verify_button = false;
        }

        let stakers_club_channel_id_ = match stakers_club_channel_id {
            Some(stakers_club_channel_id__) => stakers_club_channel_id__,
            None => {
                let mut stakers_club_permission_overwrite_registry: Vec<PermissionOverwrite> = vec![];

                '_a: for (role_id, _) in existing_guild_role_hash_map.iter() {
                    if *role_id == stakers_club_member_role_id_ {
                        stakers_club_permission_overwrite_registry.push(
                            PermissionOverwrite {
                                allow: Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY,
                                deny: Permissions::all(),
                                kind: PermissionOverwriteType::Role(*role_id),
                            },
                        )
                    } else {
                        stakers_club_permission_overwrite_registry.push(
                            PermissionOverwrite {
                                allow: Permissions::empty(),
                                deny: Permissions::all(),
                                kind: PermissionOverwriteType::Role(*role_id),
                            },
                        )
                    }
                }

                let create_channel = CreateChannel::new(Self::DISCORD_GUILD_CHANNEL_STAKERS_CLUB_NAME)
                    .kind(ChannelType::Text)
                    .permissions(stakers_club_permission_overwrite_registry);

                let guild_channel = match guild_id
                    .create_channel(
                        &http,
                        create_channel,
                    )
                    .await
                {
                    Ok(guild_channel_) => guild_channel_,
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

                guild_channel.id
            }
        };

        if let Err(mut error_auditor) = Self::create_command_initiate_ruffle(
            guild_id, &http,
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

        if let Err(mut error_auditor) = Self::create_command_cancel_raffle(
            guild_id, &http,
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

        if let Err(mut error_auditor) = Self::create_command_complete_raffle(
            guild_id, &http,
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

        if let Err(mut error_auditor) = Self::create_command_update_raffle(
            guild_id, &http,
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

        if let Err(mut error_auditor) = Self::create_command_raffle_statistic(
            guild_id, &http,
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

        if let Err(mut error_auditor) = Self::create_command_remove_from_blacklist(
            guild_id, &http,
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

        let shared_data = Arc::new(
            SharedData {
                environment_configuration,
                discord_guild_id: guild_id,
                discord_guild_wallet_verified_role_id: wallet_verified_role_id_,
                discord_guild_stakers_club_member_role_id: stakers_club_member_role_id_,
                discord_guild_wallet_verification_channel_id: wallet_verification_channel_id_,
                discord_guild_stakers_club_channel_id: stakers_club_channel_id_,
                recipient_hd_wallet_id,
                sender_hd_wallet_id,
                postgresql_connection_pool,
                stake_treshold_quantity_for_stakers_club_role,
                wallet_verification_flow_previous_undeleted_component_interaction_response_hash_map: Arc::new(Mutex::new(HashMap::new())),
                raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map: Arc::new(Mutex::new(HashMap::new())),
            },
        );

        Spawner::<TokioNonBlockingTask>::spawn_into_background(Self::process_2(shared_data.clone()));

        Spawner::<TokioNonBlockingTask>::spawn_into_background(Processor::<UpdateDiscordRoles>::process(shared_data.clone()));

        if let Some(raffle_) = raffle {
            if !Processor::<ServeRaffle>::is_process_exist().await {
                match raffle_.status {
                    Raffle_Status::ParticipantsRecruitment => {
                        Spawner::<TokioNonBlockingTask>::spawn_into_background(
                            Processor::<ServeRaffle>::process(
                                shared_data.clone(),
                                Arc::new(http),
                                Arc::new(raffle_),
                            ),
                        );
                    }
                    Raffle_Status::PrizeTransfer => {
                        todo!();
                    }
                    _ => {}
                }
            }
        }

        return Ok(());
    }

    async fn process_2(shared_data: Arc<SharedData>) -> Result<(), Auditor<Error>> {
        '_a: loop {
            if let Err(mut error_auditor) = Self::process_3(shared_data.clone()).await {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                Logger::<Auditor<Error>>::log(&error_auditor);

                sleep(Duration::from_secs(5)).await;
            }
        }

        return Ok(());
    }

    async fn process_3(shared_data: Arc<SharedData>) -> Result<(), Auditor<Error>> {
        let gateway_intents = GatewayIntents::DIRECT_MESSAGES;

        let http = Resolver::<Http>::create(shared_data.as_ref().environment_configuration.as_ref());

        let mut client = match ClientBuilder::new_with_http(
            http,
            gateway_intents,
        )
        .type_map_insert::<SharedData>(shared_data)
        .event_handler(BaseEventHandler)
        .await
        {
            Ok(client) => client,
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

        if let Err(error) = client.start().await {
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

    async fn create_command_remove_from_blacklist<'a>(
        guild_id: GuildId,
        http: &'a Http,
    ) -> Result<(), Auditor<Error>> {
        let create_create_command_option_bech32_address = CreateCommandOption::new(
            CommandOptionType::String,
            "bech32_address",
            "Bech32-address that will be removed.",
        )
        .required(true);

        let create_command = CreateCommand::new(Self::COMMAND_REMOVE_FROM_BLACKLIST)
            .description("Removes bech32-address from blacklist.")
            .add_option(create_create_command_option_bech32_address)
            .default_member_permissions(Permissions::ADMINISTRATOR);

        if let Err(error) = guild_id
            .create_command(
                http,
                create_command,
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

    async fn create_command_initiate_ruffle<'a>(
        guild_id: GuildId,
        http: &'a Http,
    ) -> Result<(), Auditor<Error>> {
        let create_command_option_prize_amount = CreateCommandOption::new(
            CommandOptionType::Integer,
            "prize_amount",
            "ISLM quantity for prize.",
        )
        .required(true);

        let create_command_option_winners_number = CreateCommandOption::new(
            CommandOptionType::Integer,
            "winners_number",
            "Number of winners.",
        )
        .required(true);

        let create_command_option_duration = CreateCommandOption::new(
            CommandOptionType::Integer,
            "duration",
            "Number of HOURS from current moment during which participants will be recruited.",
        )
        .required(true);

        let create_command = CreateCommand::new(Self::COMMAND_INITIATE_RAFFLE)
            .description("Initiates the raffle.")
            .add_option(create_command_option_prize_amount)
            .add_option(create_command_option_winners_number)
            .add_option(create_command_option_duration)
            .default_member_permissions(Permissions::ADMINISTRATOR);

        if let Err(error) = guild_id
            .create_command(
                http,
                create_command,
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

    async fn create_command_raffle_statistic<'a>(
        guild_id: GuildId,
        http: &'a Http,
    ) -> Result<(), Auditor<Error>> {
        let create_command_option = CreateCommandOption::new(
            CommandOptionType::Integer,
            "raffle_id",
            "Raffle id.",
        )
        .required(true);

        let create_command = CreateCommand::new(Self::COMMAND_RAFFLE_STATISTIC)
            .description("Takes raffle statistic.")
            .add_option(create_command_option)
            .default_member_permissions(Permissions::ADMINISTRATOR);

        if let Err(error) = guild_id
            .create_command(
                http,
                create_command,
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

    async fn create_command_cancel_raffle<'a>(
        guild_id: GuildId,
        http: &'a Http,
    ) -> Result<(), Auditor<Error>> {
        let create_command = CreateCommand::new(Self::COMMAND_CANCEL_RAFFLE)
            .description("Cancels the current raffle.")
            .default_member_permissions(Permissions::ADMINISTRATOR);

        if let Err(error) = guild_id
            .create_command(
                http,
                create_command,
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

    async fn create_command_complete_raffle<'a>(
        guild_id: GuildId,
        http: &'a Http,
    ) -> Result<(), Auditor<Error>> {
        let create_command = CreateCommand::new(Self::COMMAND_COMPLETE_RAFFLE)
            .description("Completes the current raffle.")
            .default_member_permissions(Permissions::ADMINISTRATOR);

        if let Err(error) = guild_id
            .create_command(
                http,
                create_command,
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

    async fn create_command_update_raffle<'a>(
        guild_id: GuildId,
        http: &'a Http,
    ) -> Result<(), Auditor<Error>> {
        let create_command_option_prize_amount = CreateCommandOption::new(
            CommandOptionType::Integer,
            "prize_amount",
            "ISLM quantity for prize.",
        )
        .required(false);

        let create_command_option_winners_number = CreateCommandOption::new(
            CommandOptionType::Integer,
            "winners_number",
            "Number of winners.",
        )
        .required(false);

        let create_command_option_duration = CreateCommandOption::new(
            CommandOptionType::Integer,
            "duration",
            "Number of HOURS from current moment during which participants will be recruited.",
        )
        .required(false);

        let create_command = CreateCommand::new(Self::COMMAND_UPDATE_RAFFLE)
            .description("Updates the uncompleted raffle.")
            .add_option(create_command_option_prize_amount)
            .add_option(create_command_option_winners_number)
            .add_option(create_command_option_duration)
            .default_member_permissions(Permissions::ADMINISTRATOR);

        if let Err(error) = guild_id
            .create_command(
                http,
                create_command,
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
}

pub struct SharedData {
    pub environment_configuration: Arc<EnvironmentConfiguration>,
    pub discord_guild_id: GuildId,
    pub discord_guild_wallet_verified_role_id: RoleId,
    pub discord_guild_stakers_club_member_role_id: RoleId,
    pub discord_guild_wallet_verification_channel_id: ChannelId,
    pub discord_guild_stakers_club_channel_id: ChannelId,
    pub recipient_hd_wallet_id: i64,
    pub sender_hd_wallet_id: i64,
    pub postgresql_connection_pool: Arc<PostgresqlConnectionPoolNoTls>,
    pub stake_treshold_quantity_for_stakers_club_role: U256,

    // TODO TODO TODO
    // FallowUp тоже может быть - удалять
    // удалять по времени некоммуницированные сообщения
    pub wallet_verification_flow_previous_undeleted_component_interaction_response_hash_map: Arc<Mutex<HashMap<UserId, Arc<UndeletedComponentInteractionResponse>>>>,
    pub raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map: Arc<Mutex<HashMap<UserId, Arc<ComponentInteraction>>>>,
}

impl TypeMapKey for SharedData {
    type Value = Arc<Self>;
}

pub enum UndeletedComponentInteractionResponse {
    InteractionResponse {
        component_interaction: ComponentInteraction,
    },
    InteractionResponseFollowup {
        component_interaction: ComponentInteraction,
        message_id: MessageId,
    },
}

struct BaseEventHandler;

#[async_trait]
impl EventHandler for BaseEventHandler {
    async fn interaction_create<'a>(
        &'a self,
        context: Context,
        interaction: Interaction,
    ) -> () {
        if let Err(mut error_auditor) = Self::interaction_create_(
            context,
            interaction,
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
        }

        return ();
    }

    async fn message<'a>(
        &'a self,
        context: Context,
        message: Message,
    ) -> () {
        if let Err(mut error_auditor) = Self::message_(
            context, message,
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
        }

        return ();
    }

    async fn ready<'a>(
        &'a self,
        _: Context,
        _: Ready,
    ) -> () {
        Logger::<Common>::log_info("Successfully connected to Discord.");

        return ();
    }
}

impl BaseEventHandler {
    async fn interaction_create_(
        context: Context,
        interaction: Interaction,
    ) -> Result<(), Auditor<Error>> {
        match interaction {
            Interaction::Component(component_interaction) => {
                let shared_data = {
                    let rw_lock_read_guard = context.data.read().await;

                    let shared_data_ = match rw_lock_read_guard.get::<SharedData>() {
                        Some(shared_data__) => shared_data__,
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

                    shared_data_.clone()
                };

                let shared_data_ = shared_data.as_ref();

                if component_interaction.channel_id == shared_data_.discord_guild_wallet_verification_channel_id {
                    if component_interaction.data.custom_id.as_str() == Processor::<RunBot>::CUSTOM_ID_1
                        || component_interaction.data.custom_id.as_str() == Processor::<RunBot>::CUSTOM_ID_2
                    {
                        if let Err(mut error_auditor) = Self::verify_wallet(
                            &context,
                            component_interaction,
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
                    } else {
                        return Err(
                            Auditor::<Error>::new(
                                Error::create_unreachable_state(),
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                ),
                            ),
                        );
                    }
                } else {
                    if component_interaction.channel_id == shared_data_.discord_guild_stakers_club_channel_id {
                        match Resolver::<DiscordCompositeCustomId>::from_id_to_raffle_id(
                            Processor::<RunBot>::CUSTOM_ID_3,
                            component_interaction.data.custom_id.as_str(),
                        ) {
                            Ok(raffle_id) => {
                                if let Err(mut error_auditor) = Self::participate_in_raffle(
                                    &context,
                                    component_interaction,
                                    raffle_id,
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
                            Err(_) => {
                                return Err(
                                    Auditor::<Error>::new(
                                        Error::create_unreachable_state(),
                                        BacktracePart::new(
                                            line!(),
                                            file!(),
                                        ),
                                    ),
                                );
                            }
                        }
                    }
                }
            }
            Interaction::Command(command_interaction) => {
                let command_name = command_interaction.data.name.as_str();

                let is_accessible = match command_interaction.member {
                    Some(ref member_) => {
                        let permissions = match member_.permissions {
                            Some(ref permissions_) => permissions_,
                            None => {
                                return Err(
                                    Auditor::<Error>::new(
                                        Error::create_unreachable_state(),
                                        BacktracePart::new(
                                            line!(),
                                            file!(),
                                        ),
                                    ),
                                );
                            }
                        };

                        permissions.administrator()
                    }
                    None => {
                        return Err(
                            Auditor::<Error>::new(
                                Error::create_unreachable_state(),
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                ),
                            ),
                        );
                    }
                };

                if is_accessible {
                    if command_name == Processor::<RunBot>::COMMAND_REMOVE_FROM_BLACKLIST {
                        if let Err(mut error_auditor) = Self::remove_from_blacklist(
                            &context,
                            &command_interaction,
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

                    if command_name == Processor::<RunBot>::COMMAND_INITIATE_RAFFLE {
                        if let Err(mut error_auditor) = Self::initiate_raffle(
                            &context,
                            &command_interaction,
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

                    if command_name == Processor::<RunBot>::COMMAND_CANCEL_RAFFLE {
                        if let Err(mut error_auditor) = Self::cancel_raffle(
                            &context,
                            &command_interaction,
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

                    if command_name == Processor::<RunBot>::COMMAND_COMPLETE_RAFFLE {
                        if let Err(mut error_auditor) = Self::complete_raffle(
                            &context,
                            &command_interaction,
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

                    if command_name == Processor::<RunBot>::COMMAND_UPDATE_RAFFLE {
                        if let Err(mut error_auditor) = Self::update_raffle(
                            &context,
                            &command_interaction,
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

                    if command_name == Processor::<RunBot>::COMMAND_RAFFLE_STATISTIC {
                        if let Err(mut error_auditor) = Self::raffle_statistic(
                            &context,
                            &command_interaction,
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
                }
            }
            _ => {
                return Err(
                    Auditor::<Error>::new(
                        Error::create_unreachable_state(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        }

        return Ok(());
    }

    // TODO может, зарегестрировтаь это как команду?
    // TODO может, зарегестрировтаь это как команду?
    // TODO может, зарегестрировтаь это как команду?
    // TODO может, зарегестрировтаь это как команду?
    // TODO может, зарегестрировтаь это как команду?
    async fn message_(
        context: Context,
        message: Message,
    ) -> Result<(), Auditor<Error>> {
        if message.content == Processor::<RunBot>::MESSAGE_RAFFLE_CHANCE {
            if let Err(mut error_auditor) = Self::raffle_chance(
                &context, &message,
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

        if message.content == Processor::<RunBot>::MESSAGE_MY_STAKE {
            if let Err(mut error_auditor) = Self::my_stake(
                &context, &message,
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

        return Ok(());
    }

    // TODO TODO TODO Убирать итеракшн с Куарами, закрывать спан
    // TODO TODO TODO Убирать итеракшн с Куарами, закрывать спан
    // TODO TODO TODO Убирать итеракшн с Куарами, закрывать спан
    // TODO TODO TODO Убирать итеракшн с Куарами, закрывать спан
    // TODO TODO TODO Убирать итеракшн с Куарами, закрывать спан
    async fn verify_wallet<'a>(
        context: &'a Context,
        component_interaction: ComponentInteraction,
    ) -> Result<(), Auditor<Error>> {
        let shared_data = {
            let rw_lock_read_guard = context.data.read().await;

            let shared_data_ = match rw_lock_read_guard.get::<SharedData>() {
                Some(shared_data__) => shared_data__,
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

            shared_data_.clone()
        };

        let shared_data_ = shared_data.as_ref();

        let is_shutdown_signal_received = { *IS_SHUTDOWN_SIGNAL_RECEIVED.lock().await };

        if is_shutdown_signal_received {
            let message = Processor::<RunBot>::RESPONSE_MESSAGE_FOR_SHUTDOWN_PROCESS.to_string();

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

            {
                let mut mutex_guard = shared_data_.wallet_verification_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                // TODO TODO TODO удалять без запроса, если запрос упал, то класть.
                // TODO TODO TODO удалять без запроса, если запрос упал, то класть.
                // TODO TODO TODO удалять без запроса, если запрос упал, то класть.
                // TODO TODO TODO удалять без запроса, если запрос упал, то класть.
                // TODO TODO TODO удалять без запроса, если запрос упал, то класть.
                // TODO TODO TODO удалять без запроса, если запрос упал, то класть.
                // TODO TODO TODO удалять без запроса, если запрос упал, то класть.
                // TODO TODO TODO удалять без запроса, если запрос упал, то класть.
                // TODO TODO TODO удалять без запроса, если запрос упал, то класть.
                if let Some(undeleted_component_interaction_response) = (*mutex_guard).get(&component_interaction.user.id) {
                    match *(undeleted_component_interaction_response.as_ref()) {
                        UndeletedComponentInteractionResponse::InteractionResponse {
                            component_interaction: ref component_interaction_,
                        } => {
                            if let Err(error) = component_interaction_.delete_response(context.http.as_ref()).await {
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
                                    context.http.as_ref(),
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
                };
            }

            if let Err(error) = component_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

            {
                let mut mutex_guard = shared_data_.wallet_verification_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                let _ = (*mutex_guard).insert(
                    component_interaction.user.id,
                    Arc::new(
                        UndeletedComponentInteractionResponse::InteractionResponse {
                            component_interaction,
                        },
                    ),
                );
            }

            return Ok(());
        }

        let discord_user_id = component_interaction.user.id.to_string();

        let postgresql_pooled_connection = match shared_data_.postgresql_connection_pool.as_ref().get().await {
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

        let verified_bech32_address_1_registry = match PostgresqlRepository::<VerifiedBech32Address_1>::get_all_available(
            client,
            &By_3 {
                verified_bech32_address_discord_user_id: discord_user_id.as_str(),
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

        if !verified_bech32_address_1_registry.is_empty() && component_interaction.data.custom_id.as_str() == Processor::<RunBot>::CUSTOM_ID_1 {
            let mut message = "The wallets already verified:".to_string();

            '_a: for verified_bech32_address_1 in verified_bech32_address_1_registry.into_iter() {
                message = format!(
                    "{}\n{}",
                    message.as_str(),
                    verified_bech32_address_1.value.as_str()
                );
            }

            let create_button = CreateButton::new(Processor::<RunBot>::CUSTOM_ID_2).label("Verify");

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message).button(create_button);

            {
                let mut mutex_guard = shared_data_.wallet_verification_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                if let Some(undeleted_component_interaction_response) = (*mutex_guard).get(&component_interaction.user.id) {
                    match *(undeleted_component_interaction_response.as_ref()) {
                        UndeletedComponentInteractionResponse::InteractionResponse {
                            component_interaction: ref component_interaction_,
                        } => {
                            if let Err(error) = component_interaction_.delete_response(context.http.as_ref()).await {
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
                                    context.http.as_ref(),
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
                };
            }

            if let Err(error) = component_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

            {
                let mut mutex_guard = shared_data_.wallet_verification_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                let _ = (*mutex_guard).insert(
                    component_interaction.user.id,
                    Arc::new(
                        UndeletedComponentInteractionResponse::InteractionResponse {
                            component_interaction,
                        },
                    ),
                );
            }

            return Ok(());
        }

        let recipient_hd_wallet_2 = match PostgresqlRepository::<RecipientHdWallet>::increment_mnemonic_derevation_path_index(
            client,
            By_1 {
                recipient_hd_wallet_id: shared_data_.recipient_hd_wallet_id,
            },
        )
        .await
        {
            Ok(recipient_hd_wallet_2_) => recipient_hd_wallet_2_,
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

        let recipient_hd_wallet_mnemonic_derevation_path_index = match u32::try_from(recipient_hd_wallet_2.mnemonic_derevation_path_index) {
            Ok(recipient_hd_wallet_mnemonic_derevation_path_index_) => recipient_hd_wallet_mnemonic_derevation_path_index_,
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

        let mnemonic_builder = match MnemonicBuilder::<English>::default()
            .phrase(recipient_hd_wallet_2.mnemonic_phrase.as_str())
            .index(recipient_hd_wallet_mnemonic_derevation_path_index)
        {
            Ok(mnemonic_builder_) => mnemonic_builder_,
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

        let recipient_wallet = match mnemonic_builder.build() {
            Ok(recipient_wallet_) => recipient_wallet_,
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

        let mut number = thread_rng().gen_range::<u16, _>(1000..=9999);

        let address_verification_data_expected_token_quantity = format!(
            "{}000000000000",
            number,
        );

        let address_verification_data_expected_token_quantity_human_readable = format!(
            "0.00{}",
            number
        );

        number -= 1;

        let address_verification_data_expected_inaccurate_token_quantity_human_readable = format!(
            "0.00{}",
            number
        );

        let address_verification_data_created_at = Resolver::<UTCDateTime>::get_now();

        let address_verification_data_expired_at = match Creator::<AddressVerificationData_ExpiredAt>::create(
            shared_data_.environment_configuration.as_ref(),
            address_verification_data_created_at,
        ) {
            Ok(address_verification_data_expired_at_) => address_verification_data_expired_at_,
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

        let address_verification_data_aggregate_recipient_evm_address = recipient_wallet.address().encode_hex_with_prefix();

        let address_verification_data_recipient_bech32_address = match Encoder::<Bech32>::encode(address_verification_data_aggregate_recipient_evm_address.as_str()) {
            Ok(address_verification_data_recipient_bech32_address_) => address_verification_data_recipient_bech32_address_,
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

        let address_verification_data = AddressVerificationData::new(
            discord_user_id,
            address_verification_data_recipient_bech32_address,
            address_verification_data_expected_token_quantity,
            address_verification_data_created_at,
            address_verification_data_expired_at,
        );

        if let Err(mut error_auditor) = PostgresqlRepository::<AddressVerificationData>::create(
            client,
            &address_verification_data,
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

        let evm_address_qr_code_byte_registry = match Resolver::<QrCode>::generate_byte_registry(address_verification_data_aggregate_recipient_evm_address.as_bytes()) {
            Ok(evm_address_qr_code_byte_registry_) => evm_address_qr_code_byte_registry_,
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

        let bech32_address_qr_code_byte_registry = match Resolver::<QrCode>::generate_byte_registry(address_verification_data.recipient_bech32_address.as_bytes()) {
            Ok(bech32_address_qr_code_byte_registry_) => bech32_address_qr_code_byte_registry_,
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

        let evm_address_file_name = "evm_address.png";

        let bech32_address_file_name = "bech32_address.png";

        let evm_adress_create_atachment = CreateAttachment::bytes(
            evm_address_qr_code_byte_registry,
            evm_address_file_name,
        );

        let bech32_adress_create_atachment = CreateAttachment::bytes(
            bech32_address_qr_code_byte_registry,
            bech32_address_file_name,
        );

        let evm_adress_create_embed = CreateEmbed::new()
            .attachment(evm_address_file_name)
            .title(address_verification_data_aggregate_recipient_evm_address.as_str());

        let bech32_adress_create_embed = CreateEmbed::new()
            .attachment(bech32_address_file_name)
            .title(address_verification_data.recipient_bech32_address.as_str());

        let islm_create_embed = CreateEmbed::new().title(
            format!(
                "{} ISLM",
                address_verification_data_expected_token_quantity_human_readable.as_str()
            ),
        );

        let create_interaction_response_message = CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .add_embed(islm_create_embed)
            .add_embed(evm_adress_create_embed)
            .add_file(evm_adress_create_atachment)
            .add_embed(bech32_adress_create_embed)
            .add_file(bech32_adress_create_atachment);

        {
            let mut mutex_guard = shared_data_.wallet_verification_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

            if let Some(undeleted_component_interaction_response) = (*mutex_guard).get(&component_interaction.user.id) {
                match *(undeleted_component_interaction_response.as_ref()) {
                    UndeletedComponentInteractionResponse::InteractionResponse {
                        component_interaction: ref component_interaction_,
                    } => {
                        if let Err(error) = component_interaction_.delete_response(context.http.as_ref()).await {
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
                                context.http.as_ref(),
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
            };
        }

        let process_can_not_be_interrupted_until_completed_quantity_incremented = {
            *PROCESS_CAN_NOT_BE_INTERRUPTED_UNTIL_COMPLETED_QUANTITY.lock().await += 1;

            true
        };

        if let Err(error) = component_interaction
            .create_response(
                context.http.as_ref(),
                CreateInteractionResponse::Message(create_interaction_response_message),
            )
            .await
        {
            {
                *PROCESS_CAN_NOT_BE_INTERRUPTED_UNTIL_COMPLETED_QUANTITY.lock().await -= 1;
            }

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

        {
            let mut mutex_guard = shared_data_.wallet_verification_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

            let _ = (*mutex_guard).insert(
                component_interaction.user.id,
                Arc::new(
                    UndeletedComponentInteractionResponse::InteractionResponse {
                        component_interaction: component_interaction.clone(),
                    },
                ),
            );
        }

        let address_verification_data_aggregate = AddressVerificationDataAggregate {
            address_verification_data,
            recipient_evm_address: address_verification_data_aggregate_recipient_evm_address,
            address_verification_data_expected_token_quantity_human_readable,
            address_verification_data_expected_inaccurate_token_quantity_human_readable,
        };

        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            Processor::<VerifyWallet>::process(
                process_can_not_be_interrupted_until_completed_quantity_incremented,
                address_verification_data_aggregate,
                context.http.clone(),
                component_interaction,
                shared_data.clone(),
            ),
        );

        return Ok(());
    }

    async fn participate_in_raffle<'a>(
        context: &'a Context,
        component_interaction: ComponentInteraction,
        raffle_id: i64,
    ) -> Result<(), Auditor<Error>> {
        let shared_data = {
            let rw_lock_read_guard = context.data.read().await;

            let shared_data_ = match rw_lock_read_guard.get::<SharedData>() {
                Some(shared_data__) => shared_data__,
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

            shared_data_.clone()
        };

        let shared_data_ = shared_data.as_ref();

        let user_id = component_interaction.user.id.to_string();

        let postgresql_pooled_connection = match shared_data_.postgresql_connection_pool.as_ref().get().await {
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

        let discord_user_role_1 = match PostgresqlRepository::<DiscordUserRole_1>::find(
            client,
            &By_7 {
                discord_user_role_discord_user_id: user_id.as_str(),
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

        let discord_user_role_1_ = match discord_user_role_1 {
            Some(discord_user_role_1__) => discord_user_role_1__,
            None => {
                let message = "First you must verify the wallet.";

                let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

                {
                    let mut mutex_guard = shared_data_.raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                    if let Some(component_interaction_) = (*mutex_guard).get(&component_interaction.user.id) {
                        if let Err(error) = component_interaction_.as_ref().delete_response(context.http.as_ref()).await {
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

                        let _ = (*mutex_guard).remove(&component_interaction.user.id);
                    };
                }

                if let Err(error) = component_interaction
                    .create_response(
                        context.http.as_ref(),
                        CreateInteractionResponse::Message(create_interaction_response_message),
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

                {
                    let mut mutex_guard = shared_data_.raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                    let _ = (*mutex_guard).insert(
                        component_interaction.user.id,
                        Arc::new(component_interaction),
                    );
                }

                return Ok(());
            }
        };

        if !discord_user_role_1_.stakers_club_member {
            let message = format!(
                "First you have to get the '{}' role.",
                Processor::<RunBot>::DISCORD_GUILD_ROLE_STAKERS_CLUB_MEMBER,
            );

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

            {
                let mut mutex_guard = shared_data_.raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                if let Some(component_interaction_) = (*mutex_guard).get(&component_interaction.user.id) {
                    if let Err(error) = component_interaction_.as_ref().delete_response(context.http.as_ref()).await {
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

                    let _ = (*mutex_guard).remove(&component_interaction.user.id);
                };
            }

            if let Err(error) = component_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

            {
                let mut mutex_guard = shared_data_.raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                let _ = (*mutex_guard).insert(
                    component_interaction.user.id,
                    Arc::new(component_interaction),
                );
            }

            return Ok(());
        }

        let raffle_3 = match PostgresqlRepository::<Raffle_3>::find(
            client,
            By_8 {
                raffle_id,
            },
        )
        .await
        {
            Ok(raffle_3_) => raffle_3_,
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

        let raffle_3_ = match raffle_3 {
            Some(raffle_3__) => raffle_3__,
            None => {
                let message = "The raffle is not exist.";

                let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

                {
                    let mut mutex_guard = shared_data_.raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                    if let Some(component_interaction_) = (*mutex_guard).get(&component_interaction.user.id) {
                        if let Err(error) = component_interaction_.as_ref().delete_response(context.http.as_ref()).await {
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

                        let _ = (*mutex_guard).remove(&component_interaction.user.id);
                    };
                }

                if let Err(error) = component_interaction
                    .create_response(
                        context.http.as_ref(),
                        CreateInteractionResponse::Message(create_interaction_response_message),
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

                {
                    let mut mutex_guard = shared_data_.raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                    let _ = (*mutex_guard).insert(
                        component_interaction.user.id,
                        Arc::new(component_interaction),
                    );
                }

                return Ok(());
            }
        };

        if raffle_3_.status != Raffle_Status::ParticipantsRecruitment {
            let message = "The raffle has already complited.";

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

            {
                let mut mutex_guard = shared_data_.raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                if let Some(component_interaction_) = (*mutex_guard).get(&component_interaction.user.id) {
                    if let Err(error) = component_interaction_.as_ref().delete_response(context.http.as_ref()).await {
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

                    let _ = (*mutex_guard).remove(&component_interaction.user.id);
                };
            }

            if let Err(error) = component_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

            {
                let mut mutex_guard = shared_data_.raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                let _ = (*mutex_guard).insert(
                    component_interaction.user.id,
                    Arc::new(component_interaction),
                );
            }

            return Ok(());
        }

        if (Resolver::<UTCDateTime>::get_now() + 10) > raffle_3_.expired_at {
            let message = "The time to register as a participant has already passed.";

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

            {
                let mut mutex_guard = shared_data_.raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                if let Some(component_interaction_) = (*mutex_guard).get(&component_interaction.user.id) {
                    if let Err(error) = component_interaction_.as_ref().delete_response(context.http.as_ref()).await {
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

                    let _ = (*mutex_guard).remove(&component_interaction.user.id);
                };
            }

            if let Err(error) = component_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

            {
                let mut mutex_guard = shared_data_.raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                let _ = (*mutex_guard).insert(
                    component_interaction.user.id,
                    Arc::new(component_interaction),
                );
            }

            return Ok(());
        }

        let is_exist = match PostgresqlRepository::<RaffleParticipant>::is_exist(
            client,
            &By_6 {
                raffle_participant_raffle_id: raffle_id,
                raffle_participant_discord_user_id: user_id.as_str(),
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

        if is_exist {
            let message = "You are already a participant.";

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

            {
                let mut mutex_guard = shared_data_.raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                if let Some(component_interaction_) = (*mutex_guard).get(&component_interaction.user.id) {
                    if let Err(error) = component_interaction_.as_ref().delete_response(context.http.as_ref()).await {
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

                    let _ = (*mutex_guard).remove(&component_interaction.user.id);
                };
            }

            if let Err(error) = component_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

            {
                let mut mutex_guard = shared_data_.raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                let _ = (*mutex_guard).insert(
                    component_interaction.user.id,
                    Arc::new(component_interaction),
                );
            }

            return Ok(());
        }

        let verified_bech32_address_1_registry = match PostgresqlRepository::<VerifiedBech32Address_1>::get_all_available(
            &*postgresql_pooled_connection,
            &By_3 {
                verified_bech32_address_discord_user_id: user_id.as_str(),
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

        if aislm_stake_for_all_addresses_quantity < shared_data_.stake_treshold_quantity_for_stakers_club_role {
            let stake_streshold_quantity_for_stakers_club_role = match format_units(
                shared_data_.stake_treshold_quantity_for_stakers_club_role,
                ISLM_COIN_PRECISION,
            ) {
                Ok(stake_streshold_quantity_for_stakers_club_role_) => stake_streshold_quantity_for_stakers_club_role_,
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

            let message = format!(
                "You will not be able to participate until your stake reaches {} ISLM.",
                stake_streshold_quantity_for_stakers_club_role,
            );

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

            {
                let mut mutex_guard = shared_data_.raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                if let Some(component_interaction_) = (*mutex_guard).get(&component_interaction.user.id) {
                    if let Err(error) = component_interaction_.as_ref().delete_response(context.http.as_ref()).await {
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

                    let _ = (*mutex_guard).remove(&component_interaction.user.id);
                };
            }

            if let Err(error) = component_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

            {
                let mut mutex_guard = shared_data_.raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

                let _ = (*mutex_guard).insert(
                    component_interaction.user.id,
                    Arc::new(component_interaction),
                );
            }

            return Ok(());
        }

        let raffle_participant = RaffleParticipant::new(
            raffle_id,
            user_id,
            Resolver::<UTCDateTime>::get_now(),
        );

        if let Err(mut error_auditor) = PostgresqlRepository::<RaffleParticipant>::create(
            client,
            &raffle_participant,
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

        let islm_stake_for_all_addresses_quantity = match format_units(
            aislm_stake_for_all_addresses_quantity,
            ISLM_COIN_PRECISION,
        ) {
            Ok(islm_stake_for_all_addresses_quantity_) => islm_stake_for_all_addresses_quantity_,
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

        let message = format!(
            "You have been added to the list of participants.\nYour stake is {} ISLM.",
            islm_stake_for_all_addresses_quantity
        );

        let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message.as_str());

        {
            let mut mutex_guard = shared_data_.raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

            if let Some(component_interaction_) = (*mutex_guard).get(&component_interaction.user.id) {
                if let Err(error) = component_interaction_.as_ref().delete_response(context.http.as_ref()).await {
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

                let _ = (*mutex_guard).remove(&component_interaction.user.id);
            };
        }

        if let Err(error) = component_interaction
            .create_response(
                context.http.as_ref(),
                CreateInteractionResponse::Message(create_interaction_response_message),
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

        {
            let mut mutex_guard = shared_data_.raffle_participating_flow_previous_undeleted_component_interaction_response_hash_map.as_ref().lock().await;

            let _ = (*mutex_guard).insert(
                component_interaction.user.id,
                Arc::new(component_interaction),
            );
        }

        return Ok(());
    }

    async fn remove_from_blacklist<'a>(
        context: &'a Context,
        command_interaction: &'a CommandInteraction,
    ) -> Result<(), Auditor<Error>> {
        let shared_data = {
            let rw_lock_read_guard = context.data.read().await;

            let shared_data_ = match rw_lock_read_guard.get::<SharedData>() {
                Some(shared_data__) => shared_data__,
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

            shared_data_.clone()
        };

        let shared_data_ = shared_data.as_ref();

        let resolved_option_registry = command_interaction.data.options();

        let bech32_address_resolved_option = match resolved_option_registry.get(0) {
            Some(bech32_address_resolved_option_) => bech32_address_resolved_option_,
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

        let bech32_address = if bech32_address_resolved_option.name == "bech32_address" {
            let bech32_address_ = match bech32_address_resolved_option.value {
                ResolvedValue::String(bech32_address__) => bech32_address__.to_string(),
                _ => {
                    return Err(
                        Auditor::<Error>::new(
                            Error::create_unreachable_state(),
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
            };

            bech32_address_
        } else {
            return Err(
                Auditor::<Error>::new(
                    Error::create_unreachable_state(),
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        };

        let postgresql_pooled_connection = match shared_data_.postgresql_connection_pool.as_ref().get().await {
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

        if let Err(error) = PostgresqlRepository::<VerifiedAddressBlacklist>::delete(
            client,
            &By_4 {
                verified_address_blacklist_bech32_address: bech32_address.as_str(),
            },
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

        let message = "Success.";

        let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

        if let Err(error) = command_interaction
            .create_response(
                context.http.as_ref(),
                CreateInteractionResponse::Message(create_interaction_response_message),
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

    async fn initiate_raffle<'a>(
        context: &'a Context,
        command_interaction: &'a CommandInteraction,
    ) -> Result<(), Auditor<Error>> {
        let is_shutdown_signal_received = { *IS_SHUTDOWN_SIGNAL_RECEIVED.lock().await };

        if is_shutdown_signal_received {
            let message = Processor::<RunBot>::RESPONSE_MESSAGE_FOR_SHUTDOWN_PROCESS.to_string();

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

            if let Err(error) = command_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

        let resolved_option_registry = command_interaction.data.options();

        let prize_amount_resolved_option = match resolved_option_registry.get(0) {
            Some(prize_amount_resolved_option_) => prize_amount_resolved_option_,
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

        let prize_amount = if prize_amount_resolved_option.name == "prize_amount" {
            let prize_amount_ = match prize_amount_resolved_option.value {
                ResolvedValue::Integer(prize_amount__) => prize_amount__,
                _ => {
                    return Err(
                        Auditor::<Error>::new(
                            Error::create_unreachable_state(),
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
            };

            prize_amount_
        } else {
            return Err(
                Auditor::<Error>::new(
                    Error::create_unreachable_state(),
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        };

        let winners_number_resolved_option = match resolved_option_registry.get(1) {
            Some(winners_number_resolved_option_) => winners_number_resolved_option_,
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

        let winners_number = if winners_number_resolved_option.name == "winners_number" {
            let winners_number_ = match winners_number_resolved_option.value {
                ResolvedValue::Integer(winners_number__) => winners_number__,
                _ => {
                    return Err(
                        Auditor::<Error>::new(
                            Error::create_unreachable_state(),
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
            };

            winners_number_
        } else {
            return Err(
                Auditor::<Error>::new(
                    Error::create_unreachable_state(),
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        };

        let duration_resolved_option = match resolved_option_registry.get(2) {
            Some(duration_resolved_option_) => duration_resolved_option_,
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

        let duration = if duration_resolved_option.name == "duration" {
            let duration_ = match duration_resolved_option.value {
                ResolvedValue::Integer(duration__) => duration__,
                _ => {
                    return Err(
                        Auditor::<Error>::new(
                            Error::create_unreachable_state(),
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
            };

            duration_
        } else {
            return Err(
                Auditor::<Error>::new(
                    Error::create_unreachable_state(),
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        };

        if prize_amount <= 0 || winners_number <= 0 || duration <= 0 {
            let message = "No parameter should be less or equal to zero.";

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

            if let Err(error) = command_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

        let shared_data = {
            let rw_lock_read_guard = context.data.read().await;

            let shared_data_ = match rw_lock_read_guard.get::<SharedData>() {
                Some(shared_data__) => shared_data__,
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

            shared_data_.clone()
        };

        let shared_data_ = shared_data.as_ref();

        let postgresql_pooled_connection = match shared_data_.postgresql_connection_pool.as_ref().get().await {
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

        let is_raffle_exist = match PostgresqlRepository::<Raffle>::is_exist_in_progress(client).await {
            Ok(is_raffle_exist_) => is_raffle_exist_,
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

        if is_raffle_exist || Processor::<ServeRaffle>::is_process_exist().await {
            let message = "There is already an uncompleted raffle.";

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

            if let Err(error) = command_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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
        } else {
            let raffle_created_at = Resolver::<UTCDateTime>::get_now();

            let raffle_expired_at = match raffle_created_at.checked_add(60 * 60 * duration) {
                Some(raffle_expired_at_) => raffle_expired_at_,
                None => {
                    return Err(
                        Auditor::<Error>::new(
                            Error::create_overflow_occured(),
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
            };

            let raffle_seed = Generator::<Raffle_Seed>::generate();

            // TODO TODO TODO
            // TODO TODO TODO Спросить у Сергея, нужно ли это
            // TODO TODO TODO
            let raffle_aes_key = "sdkmsldkmklsdmcklsdmcklsdmcsdmlcsmdkmcsldkc".to_string();

            // TODO TODO TODO
            // TODO TODO TODO Спросить у Сергея, нужно ли это
            // TODO TODO TODO
            let encryptred_raffle_seed = "sdckoSSIDCoJMWEOcMSKDSDCSMDCKSMDjsndjcnSDJNCkjNSDC";

            let insert_2 = Insert_2 {
                raffle_islm_prize_amount: prize_amount,
                raffle_winners_number: winners_number,
                raffle_seed,
                raffle_aes_key,
                raffle_status: Raffle_Status::ParticipantsRecruitment,
                raffle_created_at,
                raffle_expired_at,
            };

            let raffle = match PostgresqlRepository::<Raffle>::create(
                client, insert_2,
            )
            .await
            {
                Ok(raffle_) => raffle_,
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

            let raffle_expired_at_ = match Resolver::<UTCDateTime>::from_unixtime_to_timestamp(raffle.expired_at) {
                Ok(raffle_expired_at__) => raffle_expired_at__,
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

            let message = format!(
                "The raffle begins!\nId: {}\nPrize: {} ISLM\nWinners number: {}\nWill complited at: {}",
                raffle.id, raffle.islm_prize_amount, raffle.winners_number, raffle_expired_at_,
            );

            let create_button = CreateButton::new(
                Resolver::<DiscordCompositeCustomId>::from_raffle_id_to_id(
                    Processor::<RunBot>::CUSTOM_ID_3,
                    raffle.id,
                ),
            )
            .label("Participate");

            let create_message = CreateMessage::new().content(message).button(create_button);

            if let Err(error) = shared_data_
                .discord_guild_stakers_club_channel_id
                .send_message(
                    context.http.as_ref(),
                    create_message,
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

            let message = "Success.";

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

            // TODO TOOD TODO что если вот здесь не пройдет - нужно взять в спавн в бэкграунд.
            // TODO TOOD TODO что если вот здесь не пройдет - нужно взять в спавн в бэкграунд.
            // TODO TOOD TODO что если вот здесь не пройдет - нужно взять в спавн в бэкграунд.
            // TODO TOOD TODO что если вот здесь не пройдет - нужно взять в спавн в бэкграунд.
            if let Err(error) = command_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                Processor::<ServeRaffle>::process(
                    shared_data.clone(),
                    context.http.clone(),
                    Arc::new(raffle),
                ),
            );
        }

        return Ok(());
    }

    async fn cancel_raffle<'a>(
        context: &'a Context,
        command_interaction: &'a CommandInteraction,
    ) -> Result<(), Auditor<Error>> {
        let is_shutdown_signal_received = { *IS_SHUTDOWN_SIGNAL_RECEIVED.lock().await };

        if is_shutdown_signal_received {
            let message = Processor::<RunBot>::RESPONSE_MESSAGE_FOR_SHUTDOWN_PROCESS.to_string();

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

            if let Err(error) = command_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

        let shared_data = {
            let rw_lock_read_guard = context.data.read().await;

            let shared_data_ = match rw_lock_read_guard.get::<SharedData>() {
                Some(shared_data__) => shared_data__,
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

            shared_data_.clone()
        };

        let shared_data_ = shared_data.as_ref();

        let postgresql_pooled_connection = match shared_data_.postgresql_connection_pool.as_ref().get().await {
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

        let raffle_1 = match PostgresqlRepository::<Raffle_1>::find_in_progress(&*postgresql_pooled_connection).await {
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

        let raffle_1_ = match raffle_1 {
            Some(raffle_1__) => raffle_1__,
            None => {
                let message = "There is no an uncompleted raffle.";

                let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

                if let Err(error) = command_interaction
                    .create_response(
                        context.http.as_ref(),
                        CreateInteractionResponse::Message(create_interaction_response_message),
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
        };

        if Resolver::<UTCDateTime>::get_now() >= raffle_1_.expired_at {
            let message = "The process of selecting the winners is already underway.";

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

            if let Err(error) = command_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

        let message = "The canceling process has begun and will take some time.";

        let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

        if let Err(error) = command_interaction
            .create_response(
                context.http.as_ref(),
                CreateInteractionResponse::Message(create_interaction_response_message),
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

        let raffle_4 = Raffle_4 {
            id: raffle_1_.id,
        };

        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            Processor::<CancelRaffle>::process(
                shared_data.clone(),
                context.http.clone(),
                raffle_4,
            ),
        );

        return Ok(());
    }

    async fn complete_raffle<'a>(
        context: &'a Context,
        command_interaction: &'a CommandInteraction,
    ) -> Result<(), Auditor<Error>> {
        let is_shutdown_signal_received = { *IS_SHUTDOWN_SIGNAL_RECEIVED.lock().await };

        if is_shutdown_signal_received {
            let message = Processor::<RunBot>::RESPONSE_MESSAGE_FOR_SHUTDOWN_PROCESS.to_string();

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

            if let Err(error) = command_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

        let shared_data = {
            let rw_lock_read_guard = context.data.read().await;

            let shared_data_ = match rw_lock_read_guard.get::<SharedData>() {
                Some(shared_data__) => shared_data__,
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

            shared_data_.clone()
        };

        let shared_data_ = shared_data.as_ref();

        let postgresql_pooled_connection = match shared_data_.postgresql_connection_pool.as_ref().get().await {
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

        let raffle = match PostgresqlRepository::<Raffle>::find_in_progress(&*postgresql_pooled_connection).await {
            Ok(raffle_) => raffle_,
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

        let raffle_ = match raffle {
            Some(raffle__) => raffle__,
            None => {
                let message = "There is no an uncompleted raffle.";

                let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

                if let Err(error) = command_interaction
                    .create_response(
                        context.http.as_ref(),
                        CreateInteractionResponse::Message(create_interaction_response_message),
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
        };

        if Resolver::<UTCDateTime>::get_now() >= raffle_.expired_at {
            let message = "The process of selecting the winners is already underway.";

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

            if let Err(error) = command_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

        let message = "The compliting process has begun and will take some time.";

        let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

        if let Err(error) = command_interaction
            .create_response(
                context.http.as_ref(),
                CreateInteractionResponse::Message(create_interaction_response_message),
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

        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            Processor::<CompleteRaffle>::process(
                shared_data.clone(),
                context.http.clone(),
                Arc::new(raffle_),
            ),
        );

        return Ok(());
    }

    async fn update_raffle<'a>(
        context: &'a Context,
        command_interaction: &'a CommandInteraction,
    ) -> Result<(), Auditor<Error>> {
        let is_shutdown_signal_received = { *IS_SHUTDOWN_SIGNAL_RECEIVED.lock().await };

        if is_shutdown_signal_received {
            let message = Processor::<RunBot>::RESPONSE_MESSAGE_FOR_SHUTDOWN_PROCESS.to_string();

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

            if let Err(error) = command_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

        let mut is_invalid_parameter_exist = false;

        let resolved_option_registry = command_interaction.data.options();

        let prize_amount = match resolved_option_registry.get(0) {
            Some(prize_amount_resolved_option) => {
                let prize_amount_ = if prize_amount_resolved_option.name == "prize_amount" {
                    let prize_amount__ = match prize_amount_resolved_option.value {
                        ResolvedValue::Integer(prize_amount__) => {
                            if prize_amount__ <= 0 {
                                is_invalid_parameter_exist = true;
                            }

                            prize_amount__
                        }
                        _ => {
                            return Err(
                                Auditor::<Error>::new(
                                    Error::create_unreachable_state(),
                                    BacktracePart::new(
                                        line!(),
                                        file!(),
                                    ),
                                ),
                            );
                        }
                    };

                    prize_amount__
                } else {
                    return Err(
                        Auditor::<Error>::new(
                            Error::create_unreachable_state(),
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                };

                Some(prize_amount_)
            }
            None => None,
        };

        let winners_number = match resolved_option_registry.get(1) {
            Some(winners_number_resolved_option) => {
                let winners_number_ = if winners_number_resolved_option.name == "winners_number" {
                    let winners_number__ = match winners_number_resolved_option.value {
                        ResolvedValue::Integer(winners_number__) => {
                            if winners_number__ <= 0 {
                                is_invalid_parameter_exist = true;
                            }

                            winners_number__
                        }
                        _ => {
                            return Err(
                                Auditor::<Error>::new(
                                    Error::create_unreachable_state(),
                                    BacktracePart::new(
                                        line!(),
                                        file!(),
                                    ),
                                ),
                            );
                        }
                    };

                    winners_number__
                } else {
                    return Err(
                        Auditor::<Error>::new(
                            Error::create_unreachable_state(),
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                };

                Some(winners_number_)
            }
            None => None,
        };

        let duration = match resolved_option_registry.get(2) {
            Some(duration_resolved_option) => {
                let duration_ = if duration_resolved_option.name == "duration" {
                    let duration__ = match duration_resolved_option.value {
                        ResolvedValue::Integer(duration__) => {
                            if duration__ <= 0 {
                                is_invalid_parameter_exist = true;
                            }

                            duration__
                        }
                        _ => {
                            return Err(
                                Auditor::<Error>::new(
                                    Error::create_unreachable_state(),
                                    BacktracePart::new(
                                        line!(),
                                        file!(),
                                    ),
                                ),
                            );
                        }
                    };

                    duration__
                } else {
                    return Err(
                        Auditor::<Error>::new(
                            Error::create_unreachable_state(),
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                };

                Some(duration_)
            }
            None => None,
        };

        let mut invalid_parameter_message: Option<&'static str> = None;

        if is_invalid_parameter_exist {
            invalid_parameter_message = Some("No parameter should be less or equal to zero.");
        }

        if prize_amount.is_none() && winners_number.is_none() && duration.is_none() {
            invalid_parameter_message = Some("There must be at least one parameter.");
        }

        if let Some(invalid_parameter_message_) = invalid_parameter_message {
            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(invalid_parameter_message_);

            if let Err(error) = command_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

        let shared_data = {
            let rw_lock_read_guard = context.data.read().await;

            let shared_data_ = match rw_lock_read_guard.get::<SharedData>() {
                Some(shared_data__) => shared_data__,
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

            shared_data_.clone()
        };

        let shared_data_ = shared_data.as_ref();

        let postgresql_pooled_connection = match shared_data_.postgresql_connection_pool.as_ref().get().await {
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

        let raffle = match PostgresqlRepository::<Raffle>::find_in_progress(&*postgresql_pooled_connection).await {
            Ok(raffle_) => raffle_,
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

        todo!("check raffle status ");
        let raffle_ = match raffle {
            Some(raffle__) => raffle__,
            None => {
                let message = "There is no an uncompleted raffle.";

                let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

                if let Err(error) = command_interaction
                    .create_response(
                        context.http.as_ref(),
                        CreateInteractionResponse::Message(create_interaction_response_message),
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
        };

        if Resolver::<UTCDateTime>::get_now() >= raffle_.expired_at {
            let message = "The process of selecting the winners is already underway.";

            let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

            if let Err(error) = command_interaction
                .create_response(
                    context.http.as_ref(),
                    CreateInteractionResponse::Message(create_interaction_response_message),
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

        let mut update_5 = Update_5 {
            raffle_islm_prize_amount: raffle_.islm_prize_amount,
            raffle_winners_number: raffle_.winners_number,
            raffle_expired_at: raffle_.expired_at,
        };

        if let Some(prize_amount_) = prize_amount {
            update_5.raffle_islm_prize_amount = prize_amount_;
        }

        if let Some(winners_number_) = winners_number {
            update_5.raffle_winners_number = winners_number_;
        }

        if let Some(duration_) = duration {
            update_5.raffle_expired_at = match Resolver::<UTCDateTime>::get_now().checked_add(60 * 60 * duration_) {
                Some(raffle_expired_at_) => raffle_expired_at_,
                None => {
                    return Err(
                        Auditor::<Error>::new(
                            Error::create_overflow_occured(),
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
            };
        }

        let message = "The updating process has begun and will take some time.";

        let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message);

        if let Err(error) = command_interaction
            .create_response(
                context.http.as_ref(),
                CreateInteractionResponse::Message(create_interaction_response_message),
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

        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            Processor::<UpdateRaffle>::process(
                shared_data.clone(),
                context.http.clone(),
                raffle_.id,
                update_5,
            ),
        );

        return Ok(());
    }

    async fn raffle_statistic<'a>(
        context: &'a Context,
        command_interaction: &'a CommandInteraction,
    ) -> Result<(), Auditor<Error>> {
        let resolved_option_registry = command_interaction.data.options();

        let raffle_id_resolved_option = match resolved_option_registry.get(0) {
            Some(raffle_id_resolved_option_) => raffle_id_resolved_option_,
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

        let raffle_id = if raffle_id_resolved_option.name == "raffle_id" {
            let raffle_id_ = match raffle_id_resolved_option.value {
                ResolvedValue::Integer(raffle_id__) => raffle_id__,
                _ => {
                    return Err(
                        Auditor::<Error>::new(
                            Error::create_unreachable_state(),
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
            };

            raffle_id_
        } else {
            return Err(
                Auditor::<Error>::new(
                    Error::create_unreachable_state(),
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        };

        let shared_data = {
            let rw_lock_read_guard = context.data.read().await;

            let shared_data_ = match rw_lock_read_guard.get::<SharedData>() {
                Some(shared_data__) => shared_data__,
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

            shared_data_.clone()
        };

        let shared_data_ = shared_data.as_ref();

        let postgresql_pooled_connection = match shared_data_.postgresql_connection_pool.as_ref().get().await {
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

        let mut is_stake_updating_subprocess_exist = false;

        let mut aislm_stake_1_registry_registry: Vec<Vec<AislmStake_1>> = vec![];

        let mut aislm_stake_id: Option<i64> = None;

        let limit = 10000;

        'a: loop {
            {
                // is_stake_updating_subprocess_exist = *IS_STAKE_UPDATING_SUBPROCESS_EXIST.lock().await;
                todo!()
            }

            // TODO TODO TODO
            // TODO TODO TODO
            // TODO TODO TODO
            // TODO TODO TODO показывать только те, где обновление_финишед. вот это убрать
            if is_stake_updating_subprocess_exist {
                let message_ = "The data is being updated, please try again in a couple of minutes.";

                let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).content(message_);

                if let Err(error) = command_interaction
                    .create_response(
                        context.http.as_ref(),
                        CreateInteractionResponse::Message(create_interaction_response_message),
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

            let aislm_stake_1_registry = match PostgresqlRepository::<AislmStake_1>::get_all(
                &*postgresql_pooled_connection,
                By_9 {
                    aislm_stake_raffle_id: raffle_id,
                },
                aislm_stake_id,
                limit,
            )
            .await
            {
                Ok(aislm_stake_1_registry_) => aislm_stake_1_registry_,
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

            if aislm_stake_1_registry.is_empty() {
                break 'a;
            }

            let aislm_stake_1_registry_length = aislm_stake_1_registry.len();

            aislm_stake_id = Some(aislm_stake_1_registry[aislm_stake_1_registry_length - 1].id);

            aislm_stake_1_registry_registry.push(aislm_stake_1_registry);

            if aislm_stake_1_registry_length < (limit as usize) {
                break 'a;
            }
        }

        let mut writer = csv::Writer::from_writer(Vec::<u8>::new());

        if let Err(error) = writer.write_field("discord_id") {
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

        if let Err(error) = writer.write_field("evm_address") {
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

        if let Err(error) = writer.write_field("amount") {
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

        if let Err(error) = writer.write_field("created_at") {
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

        if let Err(error) = writer.write_record(None::<&[u8]>) {
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

        '_a: for aislm_stake_1_registry in aislm_stake_1_registry_registry.into_iter() {
            '_b: for aislm_stake_1 in aislm_stake_1_registry.into_iter() {
                if let Err(error) = writer.write_field(aislm_stake_1.discord_user_id) {
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

                if let Err(error) = writer.write_field(aislm_stake_1.bech32_address) {
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

                if let Err(error) = writer.write_field(aislm_stake_1.amount) {
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

                let created_at = match Resolver::<UTCDateTime>::from_unixtime_to_timestamp(aislm_stake_1.created_at) {
                    Ok(created_at_) => created_at_,
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

                if let Err(error) = writer.write_field(created_at) {
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

                if let Err(error) = writer.write_record(None::<&'_ [u8]>) {
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

        let data = match writer.into_inner() {
            Ok(data_) => data_,
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

        let file_name = "statistic.csv";

        let create_atachment = CreateAttachment::bytes(
            data, file_name,
        );

        let create_interaction_response_message = CreateInteractionResponseMessage::new().ephemeral(true).add_file(create_atachment);

        if let Err(error) = command_interaction
            .create_response(
                context.http.as_ref(),
                CreateInteractionResponse::Message(create_interaction_response_message),
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

    async fn raffle_chance<'a>(
        context: &'a Context,
        message: &'a Message,
    ) -> Result<(), Auditor<Error>> {
        let shared_data = {
            let rw_lock_read_guard = context.data.read().await;

            let shared_data_ = match rw_lock_read_guard.get::<SharedData>() {
                Some(shared_data__) => shared_data__,
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

            shared_data_.clone()
        };

        let shared_data_ = shared_data.as_ref();

        let postgresql_pooled_connection = match shared_data_.postgresql_connection_pool.as_ref().get().await {
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
        let raffle_1_ = match raffle_1 {
            Some(raffle_1__) => raffle_1__,
            None => {
                let message_ = "The raffle does not exist.";

                if let Err(error) = message
                    .channel_id
                    .say(
                        context.http.as_ref(),
                        message_,
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
        };

        let is_exist = match PostgresqlRepository::<RaffleParticipant>::is_exist(
            client,
            &By_6 {
                raffle_participant_raffle_id: raffle_1_.id,
                raffle_participant_discord_user_id: message.author.id.to_string().as_str(),
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
            let message_ = "You are not participant.";

            if let Err(error) = message
                .channel_id
                .say(
                    context.http.as_ref(),
                    message_,
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

        let mut is_stake_updating_subprocess_exist = false;

        let mut aislm_stake_2_registry_registry: Vec<Vec<AislmStake_2>> = vec![];

        let mut aislm_stake_id: Option<i64> = None;

        let limit = 10000;

        'a: loop {
            {
                // is_stake_updating_subprocess_exist = *IS_STAKE_UPDATING_SUBPROCESS_EXIST.lock().await;
                todo!();
            }

            if is_stake_updating_subprocess_exist {
                let message_ = "The data is being updated, please try again in a couple of minutes.";

                if let Err(error) = message
                    .channel_id
                    .say(
                        context.http.as_ref(),
                        message_,
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

            let aislm_stake_2_registry = match PostgresqlRepository::<AislmStake_2>::get_all(
                client,
                By_9 {
                    aislm_stake_raffle_id: raffle_1_.id,
                },
                aislm_stake_id,
                limit,
            )
            .await
            {
                Ok(aislm_stake_2_registry_) => aislm_stake_2_registry_,
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

            if aislm_stake_2_registry.is_empty() {
                break 'a;
            }

            let aislm_stake_2_registry_length = aislm_stake_2_registry.len();

            aislm_stake_id = Some(aislm_stake_2_registry[aislm_stake_2_registry_length - 1].id);

            aislm_stake_2_registry_registry.push(aislm_stake_2_registry);

            if aislm_stake_2_registry_length < (limit as usize) {
                break 'a;
            }
        }

        if aislm_stake_2_registry_registry.is_empty() {
            let message_ = "Information has not been updated yet.";

            if let Err(error) = message
                .channel_id
                .say(
                    context.http.as_ref(),
                    message_,
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

        let mut aislm_stake_for_all_users_quantity = U256::zero();

        let mut aislm_stake_for_current_user_quantity = U256::zero();

        '_a: for aislm_stake_2_registry in aislm_stake_2_registry_registry.into_iter() {
            '_b: for aislm_stake_2 in aislm_stake_2_registry.into_iter() {
                let aislm_stake_quantity = match U256::from_dec_str(aislm_stake_2.amount.as_str()) {
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

                if aislm_stake_2.discord_user_id == message.author.id.to_string() {
                    aislm_stake_for_current_user_quantity += aislm_stake_quantity;
                }

                aislm_stake_for_all_users_quantity += aislm_stake_quantity;
            }
        }

        if aislm_stake_for_current_user_quantity.is_zero() {
            let message_ = "The total stake amount is 0.";

            if let Err(error) = message
                .channel_id
                .say(
                    context.http.as_ref(),
                    message_,
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

        let aislm_stake_for_all_users_quantity_ = match aislm_stake_for_all_users_quantity.to_string().parse::<f64>() {
            Ok(aislm_stake_for_all_users_quantity__) => aislm_stake_for_all_users_quantity__,
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

        let aislm_stake_for_current_user_quantity_ = match aislm_stake_for_current_user_quantity.to_string().parse::<f64>() {
            Ok(aislm_stake_for_current_user_quantity__) => aislm_stake_for_current_user_quantity__,
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

        let chance = ((aislm_stake_for_current_user_quantity_ / aislm_stake_for_all_users_quantity_) * (100 as f64)).round() / (100 as f64);

        let message_ = format!(
            "{}",
            chance.to_string().as_str()
        );

        if let Err(error) = message
            .channel_id
            .say(
                context.http.as_ref(),
                message_,
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

    async fn my_stake<'a>(
        context: &'a Context,
        message: &'a Message,
    ) -> Result<(), Auditor<Error>> {
        let shared_data = {
            let rw_lock_read_guard = context.data.read().await;

            let shared_data_ = match rw_lock_read_guard.get::<SharedData>() {
                Some(shared_data__) => shared_data__,
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

            shared_data_.clone()
        };

        let shared_data_ = shared_data.as_ref();

        let postgresql_pooled_connection = match shared_data_.postgresql_connection_pool.as_ref().get().await {
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
                verified_bech32_address_discord_user_id: message.author.id.to_string().as_str(),
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

        let islm_stake_for_all_addresses_quantity = match format_units(
            aislm_stake_for_all_addresses_quantity,
            ISLM_COIN_PRECISION,
        ) {
            Ok(islm_stake_for_all_addresses_quantity_) => islm_stake_for_all_addresses_quantity_,
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

        let message_ = format!(
            "{} ISLM",
            islm_stake_for_all_addresses_quantity.as_str()
        );

        if let Err(error) = message
            .channel_id
            .say(
                context.http.as_ref(),
                message_,
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
}


продолжить разбираться с todo!() и с TODO
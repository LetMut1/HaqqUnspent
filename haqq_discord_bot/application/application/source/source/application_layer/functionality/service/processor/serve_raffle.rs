use bech32::Bech32;
use chrono::Utc;
use ethers::middleware::SignerMiddleware;
use ethers::providers::Provider;
use ethers::signers::coins_bip39::English;
use ethers::signers::MnemonicBuilder;
use ethers::signers::Signer;
use ethers::types::Address;
use ethers::types::TransactionRequest;
use ethers::utils::hex::ToHexExt;
use crate::application_layer::functionality::command_processor::run_bot::IS_SHUTDOWN_SIGNAL_RECEIVED;
use crate::domain_layer::data::entity::aislm_stake::AislmStake;
use crate::domain_layer::data::entity::prize_transfer_proof::PrizeTransferProof;
use crate::domain_layer::data::entity::raffle_winner::RaffleWinner;
use crate::domain_layer::data::entity::raffle::Raffle_2;
use crate::domain_layer::data::entity::raffle::Raffle_Status;
use crate::domain_layer::data::entity::raffle::Raffle;
use crate::domain_layer::data::entity::sender_hd_wallet::SenderHdWallet_2;
use crate::domain_layer::data::entity::verified_bech32_address::VerifiedBech32Address_3;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::control_type::Cosmos;
use crate::infrastructure_layer::data::control_type::TokioNonBlockingTask;
use crate::infrastructure_layer::data::control_type::UTCDateTime;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_10;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_8;
use crate::infrastructure_layer::functionality::repository::postgresql::insert::Insert_3;
use crate::infrastructure_layer::functionality::repository::postgresql::PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::postgresql::update::Update_4;
use crate::infrastructure_layer::functionality::service::encoder::Encoder;
use crate::infrastructure_layer::functionality::service::http_request_resolver::HttpRequestResolver;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use crate::infrastructure_layer::functionality::service::spawner::Spawner;
use crate::ISLM_COIN_PRECISION;
use cron::Schedule;
use ethers::types::U256;
use ethers::utils::format_units;
use rand::Rng;
use serenity::all::CreateMessage;
use serenity::http::Http;
use std::clone::Clone;
use tokio::task::AbortHandle;
use ethers::providers::Http as EthersProviderHttp;
use ethers::providers::Middleware;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;
use super::Processor;
use super::run_bot::SharedData;
use tokio::sync::Mutex;
use tokio::time::Duration;
use tokio::time::sleep;

pub use crate::infrastructure_layer::data::control_type::ServeRaffle;

static SERVE_RAFFLE_TASK_ABORT_HANDLE: Mutex<Option<AbortHandle>> = Mutex::const_new(None);

static UPDATE_STAKE_TASK_ABORT_HANDLE: Mutex<Option<AbortHandle>> = Mutex::const_new(None);

static FINISH_RAFFLE_TASK_ABORT_HANDLE: Mutex<Option<AbortHandle>> = Mutex::const_new(None);

impl Processor<ServeRaffle> {
    const WEIGHT_FACTOR: u16 = 1000;

    pub async fn is_process_exist() -> bool {
        let is_exist_serve_raffle_task_process = {
            (*SERVE_RAFFLE_TASK_ABORT_HANDLE.lock().await).is_some()
        };

        let is_exist_update_stake_task_process = {
            (*UPDATE_STAKE_TASK_ABORT_HANDLE.lock().await).is_some()
        };

        let is_exist_finish_raffle_task_process = {
            (*FINISH_RAFFLE_TASK_ABORT_HANDLE.lock().await).is_some()
        };

        return is_exist_serve_raffle_task_process || is_exist_update_stake_task_process || is_exist_finish_raffle_task_process;
    }

    pub async fn process(
        shared_data: Arc<SharedData>,
        http: Arc<Http>,
        raffle: Arc<Raffle>,
    ) -> Result<(), Auditor<Error>> {
        let serve_raffle_task_join_handle = Spawner::<TokioNonBlockingTask>::spawn_processed(
            Self::process_1(shared_data, http, raffle),
        );

        {
            *SERVE_RAFFLE_TASK_ABORT_HANDLE.lock().await = Some(serve_raffle_task_join_handle.abort_handle());
        }

        let _ = serve_raffle_task_join_handle.await;

        {
            *SERVE_RAFFLE_TASK_ABORT_HANDLE.lock().await = None;
        }

        return Ok(());
    }

    async fn process_1(
        shared_data: Arc<SharedData>,
        http: Arc<Http>,
        raffle: Arc<Raffle>,
    ) -> Result<(), Auditor<Error>> {
        'a: loop {
            if let Err(mut error_auditor) = Self::process_2(
                http.clone(),
                shared_data.clone(),
                raffle.clone(),
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

    async fn process_2(
        http: Arc<Http>,
        shared_data: Arc<SharedData>,
        raffle: Arc<Raffle>,
    ) -> Result<(), Auditor<Error>> {
        let schedule = match Schedule::from_str(
            shared_data
                .as_ref()
                .environment_configuration
                .as_ref()
                .noncontext_parameters
                .raffle_stake_updating_cron_configuration
                .as_str(),
        ) {
            Ok(shedule_) => shedule_,
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

        let raffle_ = raffle.as_ref();

        'a: for sheduled_date_time in schedule.upcoming(Utc) {
            let sheduled_ = sheduled_date_time.timestamp();

            if sheduled_ < raffle_.expired_at {
                '_b: while Resolver::<UTCDateTime>::get_now() < sheduled_ {
                    {
                        if *IS_SHUTDOWN_SIGNAL_RECEIVED.lock().await {
                            return Ok(());
                        }
                    }

                    sleep(Duration::from_secs(1)).await;
                }

                let is_needed_to_execute = {
                    (*UPDATE_STAKE_TASK_ABORT_HANDLE.lock().await).is_none()
                };

                if is_needed_to_execute {
                    Spawner::<TokioNonBlockingTask>::spawn_into_background(
                        Self::update_stake_and_notify(
                            http.clone(),
                            shared_data.clone(),
                            raffle.clone(),
                            sheduled_,
                        ),
                    );
                } else {
                    continue 'a;
                }
            } else {
                '_b: while Resolver::<UTCDateTime>::get_now() < raffle_.expired_at {
                    {
                        if *IS_SHUTDOWN_SIGNAL_RECEIVED.lock().await {
                            return Ok(());
                        }
                    }

                    sleep(Duration::from_secs(1)).await;
                }

                'c: loop {
                    {
                        if *IS_SHUTDOWN_SIGNAL_RECEIVED.lock().await {
                            return Ok(());
                        }
                    }

                    let is_needed_to_sleep = {
                        (*UPDATE_STAKE_TASK_ABORT_HANDLE.lock().await).is_some()
                    };

                    if is_needed_to_sleep {
                        sleep(Duration::from_secs(1)).await;

                        continue 'c;
                    } else {
                        break 'c;
                    }
                }

                let _ = Self::finish_raffle_and_notify(
                    http.clone(),
                    shared_data.clone(),
                    raffle.clone(),
                    raffle_.expired_at,
                ).await;

                break 'a;
            }
        }

        return Ok(());
    }

    async fn update_stake_and_notify(
        http: Arc<Http>,
        shared_data: Arc<SharedData>,
        raffle: Arc<Raffle>,
        sheduled_unixtime: i64,
    ) -> Result<(), Auditor<Error>> {
        let update_stake_task_join_handle = Spawner::<TokioNonBlockingTask>::spawn_processed(
            Self::update_stake_and_notify_1(http, shared_data, raffle, sheduled_unixtime),
        );

        {
            *UPDATE_STAKE_TASK_ABORT_HANDLE.lock().await = Some(update_stake_task_join_handle.abort_handle());
        }

        let _ = update_stake_task_join_handle.await;

        {
            *UPDATE_STAKE_TASK_ABORT_HANDLE.lock().await = None;
        }

        return Ok(());
    }

    async fn update_stake_and_notify_1(
        http: Arc<Http>,
        shared_data: Arc<SharedData>,
        raffle: Arc<Raffle>,
        sheduled_unixtime: i64,
    ) -> Result<(), Auditor<Error>> {
        {
            if *IS_SHUTDOWN_SIGNAL_RECEIVED.lock().await {
                return Ok(());
            }
        }

        let mut discord_user_id_stake_quantity_registry: Option<Vec<(String, U256)>> = None;

        let mut counter = 0 as usize;

        'a: loop {
            if let Err(mut error_auditor) = Self::update_stake_and_notify_2(
                http.clone(),
                shared_data.clone(),
                raffle.clone(),
                sheduled_unixtime,
                &mut discord_user_id_stake_quantity_registry,
            )
            .await
            {
                counter += 1;

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
                    sleep(Duration::from_secs(5)).await;

                    continue 'a;
                } else {
                    break 'a;
                }
            }

            break 'a;
        }

        return Ok(());
    }

    async fn update_stake_and_notify_2<'a>(
        http: Arc<Http>,
        shared_data: Arc<SharedData>,
        raffle: Arc<Raffle>,
        sheduled_unixtime: i64,
        discord_user_id_stake_quantity_registry: &'a mut Option<Vec<(String, U256)>>,
    ) -> Result<(), Auditor<Error>> {
        let discord_user_id_stake_quantity_registry_ = match discord_user_id_stake_quantity_registry {
            Some(ref discord_user_id_stake_quantity_registry__) => discord_user_id_stake_quantity_registry__.as_slice(),
            None => {
                let mut discord_user_id_stake_quantity_registry__: Vec<(String, U256)> = vec![];

                let aislm_stake_aggregated_data_ = match Self::update_stake(
                    shared_data.clone(),
                    raffle.clone(),
                    sheduled_unixtime,
                )
                .await
                {
                    Ok(aislm_stake_aggregated_data_) => aislm_stake_aggregated_data_,
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

                if aislm_stake_aggregated_data_.0.is_empty() {
                    return Ok(());
                }

                '_a: for (discord_user_id, aislm_stake_data) in aislm_stake_aggregated_data_.0.into_iter() {
                    let mut aislm_stake_for_one_user_quantity = U256::zero();

                    '_b: for (_, aislm_stake_quantity) in aislm_stake_data.into_iter() {
                        aislm_stake_for_one_user_quantity += aislm_stake_quantity;
                    }

                    discord_user_id_stake_quantity_registry__.push(
                        (
                            discord_user_id,
                            aislm_stake_for_one_user_quantity,
                        ),
                    );
                }

                discord_user_id_stake_quantity_registry__.sort_by(
                    |discord_user_id_stake_quantity_left: &'_ (
                        String,
                        U256,
                    ),
                     discord_user_id_stake_quantity_right: &'_ (
                        String,
                        U256,
                    )|
                     -> Ordering {
                        let ordering = if discord_user_id_stake_quantity_left.1 < discord_user_id_stake_quantity_right.1 {
                            Ordering::Greater
                        } else {
                            let ordering_ = if discord_user_id_stake_quantity_left.1 > discord_user_id_stake_quantity_right.1 {
                                Ordering::Less
                            } else {
                                Ordering::Equal
                            };

                            ordering_
                        };

                        ordering
                    },
                );

                *discord_user_id_stake_quantity_registry = Some(discord_user_id_stake_quantity_registry__);

                let discord_user_id_stake_quantity_registry___ = match discord_user_id_stake_quantity_registry {
                    Some(ref discord_user_id_stake_quantity_registry____) => discord_user_id_stake_quantity_registry____,
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

                discord_user_id_stake_quantity_registry___.as_slice()
            }
        };

        let mut message = "Leaderboard:".to_string();

        'a: for (index, (discord_user_id, aislm_stake_quantity)) in discord_user_id_stake_quantity_registry_.iter().enumerate() {
            if index < 30 {
                let islm_stake_quantity = match format_units(
                    *aislm_stake_quantity,
                    ISLM_COIN_PRECISION,
                ) {
                    Ok(islm_stake_quantity_) => islm_stake_quantity_,
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

                message = format!(
                    "{}\n<@{}> {} ISLM",
                    message.as_str(),
                    discord_user_id,
                    islm_stake_quantity.to_string().as_str(),
                );
            } else {
                break 'a;
            }
        }

        let create_message = CreateMessage::new().content(message);

        if let Err(error) = shared_data
            .as_ref()
            .discord_guild_stakers_club_channel_id
            .send_message(
                http.as_ref(),
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

        return Ok(());
    }

    async fn update_stake(
        shared_data: Arc<SharedData>,
        raffle: Arc<Raffle>,
        sheduled_unixtime: i64,
    ) -> Result<AislmStakeAggregatedData, Auditor<Error>> {
        // TODO TOOD TODO этот метод зациклить
        // TODO TOOD TODO этот метод зациклить
        // TODO TOOD TODO этот метод зациклить
        // TODO TOOD TODO этот метод зациклить
        // TODO TOOD TODO этот метод зациклить
        // TODO TOOD TODO этот метод зациклить



        // TODO TODO Таблица,которая фиусирует, что апдейт прошел успешно в это время. Для статистики убирать неуспешные апдейты.
        // TODO TODO Таблица,которая фиусирует, что апдейт прошел успешно в это время. Для статистики убирать неуспешные апдейты.
        // TODO TODO Таблица,которая фиусирует, что апдейт прошел успешно в это время. Для статистики убирать неуспешные апдейты.
        // TODO TODO Таблица,которая фиусирует, что апдейт прошел успешно в это время. Для статистики убирать неуспешные апдейты.
        // TODO TODO Таблица,которая фиусирует, что апдейт прошел успешно в это время. Для статистики убирать неуспешные апдейты.
        // TODO TODO Таблица,которая фиусирует, что апдейт прошел успешно в это время. Для статистики убирать неуспешные апдейты.

        let shared_data_ = shared_data.as_ref();

        let raffle_ = raffle.as_ref();

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

        let mut registry_registry: Vec<
            Vec<(
                String,
                Vec<String>,
            )>,
        > = vec![];

        let limit = 1000;

        let mut verified_bech32_address_discord_user_id: Option<String> = None;

        'a: loop {
            let verified_bech32_address_discord_user_id_ = match verified_bech32_address_discord_user_id {
                Some(ref verified_bech32_address_discord_user_id__) => Some(verified_bech32_address_discord_user_id__.as_str()),
                None => None,
            };

            let registry = match PostgresqlRepository::<VerifiedBech32Address_3>::get_all_available_for_raffle(
                client,
                By_8 {
                    raffle_id: raffle_.id,
                },
                verified_bech32_address_discord_user_id_,
                limit,
            )
            .await
            {
                Ok(registry_) => registry_,
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

            if registry.is_empty() {
                break 'a;
            }

            let registry_length = registry.len();

            verified_bech32_address_discord_user_id = Some(registry[registry_length - 1].0.clone());

            registry_registry.push(registry);

            if registry_length < (limit as usize) {
                break 'a;
            }
        }

        let mut registry_: Vec<(
            String,
            Vec<(
                String,
                U256,
            )>,
        )> = vec![];

        let mut insert_3_registry: Vec<Insert_3> = vec![];

        '_a: for registry in registry_registry.into_iter() {
            '_b: for (verified_bech32_address_discord_user_id, verified_bech32_address_value_registry) in registry.into_iter() {
                let mut verified_bech32_address_value_aislm_stake_quantity_registry_: Vec<(
                    String,
                    U256,
                )> = vec![];

                '_c: for verified_bech32_address_value in verified_bech32_address_value_registry.into_iter() {
                    let mut aislm_stake_for_one_address_quantity = U256::zero();

                    // TODO grpc?
                    // TODO grpc?
                    // TODO grpc?
                    // TODO другой алгоритм забора данных с сети, чтобы не было большго количества запросов.
                    // TODO другой алгоритм забора данных с сети, чтобы не было большго количества запросов.
                    // TODO другой алгоритм забора данных с сети, чтобы не было большго количества запросов.
                    let stake_quantity_by_address_response = match HttpRequestResolver::<Cosmos>::get_aislm_stake_quantity_by_address(verified_bech32_address_value.as_str()).await
                    {
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

                    '_d: for delegation_response in stake_quantity_by_address_response.delegation_responses.into_iter() {
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

                        aislm_stake_for_one_address_quantity += aislm_stake_quantity;
                    }

                    let aislm_stake_quantity_ = aislm_stake_for_one_address_quantity.to_string();

                    let insert_3 = Insert_3 {
                        aislm_stake_amount: aislm_stake_quantity_,
                        aislm_stake_raffle_id: raffle_.id,
                        aislm_stake_bech32_address: verified_bech32_address_value.clone(),
                        aislm_stake_discord_user_id: verified_bech32_address_discord_user_id.clone(),
                        aislm_stake_created_at: sheduled_unixtime,
                    };

                    insert_3_registry.push(insert_3);

                    verified_bech32_address_value_aislm_stake_quantity_registry_.push(
                        (
                            verified_bech32_address_value,
                            aislm_stake_for_one_address_quantity,
                        ),
                    )
                }

                registry_.push(
                    (
                        verified_bech32_address_discord_user_id,
                        verified_bech32_address_value_aislm_stake_quantity_registry_,
                    ),
                );

                if insert_3_registry.len() == 250 {
                    if let Err(mut error_auditor) = PostgresqlRepository::<AislmStake>::batch_upsert(
                        client,
                        insert_3_registry.as_slice(),
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

                    insert_3_registry = vec![];
                }
            }
        }

        if !insert_3_registry.is_empty() {
            if let Err(mut error_auditor) = PostgresqlRepository::<AislmStake>::batch_upsert(
                client,
                insert_3_registry.as_slice(),
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

        return Ok(AislmStakeAggregatedData(registry_));
    }

    pub async fn finish_raffle_and_notify(
        http: Arc<Http>,
        shared_data: Arc<SharedData>,
        raffle: Arc<Raffle>,
        sheduled_unixtime: i64,
    ) -> Result<(), Auditor<Error>> {
        let finish_raffle_task_join_handle = Spawner::<TokioNonBlockingTask>::spawn_processed(
            Self::finish_raffle_and_notify_1(http, shared_data, raffle, sheduled_unixtime)
        );

        {
            *FINISH_RAFFLE_TASK_ABORT_HANDLE.lock().await = Some(finish_raffle_task_join_handle.abort_handle());
        }

        let _ = finish_raffle_task_join_handle.await;

        {
            *FINISH_RAFFLE_TASK_ABORT_HANDLE.lock().await = None;
        }

        return Ok(());
    }

    async fn finish_raffle_and_notify_1(
        http: Arc<Http>,
        shared_data: Arc<SharedData>,
        raffle: Arc<Raffle>,
        sheduled_unixtime: i64,
    ) -> Result<(), Auditor<Error>> {
        let mut aislm_stake_aggregated_data: Option<AislmStakeAggregatedData> = None;

        'a: loop {
            if let Err(mut error_auditor) = Self::finish_raffle_and_notify_2(
                http.clone(),
                shared_data.clone(),
                raffle.clone(),
                sheduled_unixtime,
                &mut aislm_stake_aggregated_data,
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

    async fn finish_raffle_and_notify_2<'a>(
        http: Arc<Http>,
        shared_data: Arc<SharedData>,
        raffle: Arc<Raffle>,
        sheduled_unixtime: i64,
        aislm_stake_aggregated_data: &'a mut Option<AislmStakeAggregatedData>,
    ) -> Result<(), Auditor<Error>> {
        let shared_data_ = shared_data.as_ref();

        let aislm_stake_aggregated_data_ = match aislm_stake_aggregated_data {
            Some(ref aislm_stake_aggregated_data__) => aislm_stake_aggregated_data__,
            None => {
                let aislm_stake_aggregated_data__ = match Self::update_stake(
                    shared_data.clone(),
                    raffle.clone(),
                    sheduled_unixtime,
                )
                .await
                {
                    Ok(aislm_stake_aggregated_data_) => aislm_stake_aggregated_data_,
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

                *aislm_stake_aggregated_data = Some(aislm_stake_aggregated_data__);

                let aislm_stake_aggregated_data___ = match aislm_stake_aggregated_data {
                    Some(ref aislm_stake_aggregated_data____) => aislm_stake_aggregated_data____,
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

                aislm_stake_aggregated_data___
            }
        };

        let raffle_ = raffle.as_ref();

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

        if aislm_stake_aggregated_data_.0.is_empty() {
            if let Err(mut error_auditor) = PostgresqlRepository::<Raffle_2>::update(
                client,
                Update_4 {
                    raffle_status: Raffle_Status::Completed,
                },
                By_8 {
                    raffle_id: raffle_.id,
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

            let message = "No participants.";

            let create_message = CreateMessage::new().content(message);

            if let Err(error) = shared_data_
                .discord_guild_stakers_club_channel_id
                .send_message(
                    http.as_ref(),
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

            return Ok(());
        }

        let raffle_winner_registry = match Self::choose_raffle_winners(
            aislm_stake_aggregated_data_,
            raffle_,
        ) {
            Ok(raffle_winner_registry_) => raffle_winner_registry_,
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

        if raffle_winner_registry.is_empty() {
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

        // TODO ТРАНЗАКЦИЯ y 2 вниз
        // TODO ТРАНЗАКЦИЯ y 2 вниз
        // TODO ТРАНЗАКЦИЯ y 2 вниз
        // TODO ТРАНЗАКЦИЯ y 2 вниз
        // TODO ТРАНЗАКЦИЯ y 2 вниз
        if let Err(mut error_auditor) = PostgresqlRepository::<RaffleWinner>::batch_insert(
            client,
            raffle_winner_registry.as_slice(),
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

        if let Err(mut error_auditor) = PostgresqlRepository::<Raffle_2>::update(
            client,
            Update_4 {
                raffle_status: Raffle_Status::PrizeTransfer,
            },
            By_8 {
                raffle_id: raffle_.id,
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

        let _ = Self::transfer_prize_and_notify(
            http.as_ref(),
            shared_data_,
            raffle.as_ref(),
            raffle_winner_registry.as_slice(),
        ).await;

        return Ok(());
    }

    async fn transfer_prize_and_notify<'a>(
        http: &'a Http,
        shared_data: &'a SharedData,
        raffle: &'a Raffle,
        raffle_winner_registry: &'a [RaffleWinner],
    ) -> Result<(), Auditor<Error>> {
        let mut is_needed_to_transfer_prize = true;

        let mut is_needed_to_create_prize_transfer_proof = true;

        let mut prize_received_raffle_winner_discord_user_id_hash_set = HashSet::<String>::new();

        let mut prize_transfer_proof_registry: Vec<PrizeTransferProof> = vec![];

        'a: loop {
            if let Err(mut error_auditor) = Self::transfer_prize_and_notify_(
                http,
                shared_data,
                raffle,
                raffle_winner_registry,
                &mut is_needed_to_transfer_prize,
                &mut is_needed_to_create_prize_transfer_proof,
                &mut prize_received_raffle_winner_discord_user_id_hash_set,
                &mut prize_transfer_proof_registry,
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

                sleep(Duration::from_secs(1)).await;

                continue 'a;
            }

            break 'a;
        }

        return Ok(());
    }

    async fn transfer_prize_and_notify_<'a>(
        http: &'a Http,
        shared_data: &'a SharedData,
        raffle: &'a Raffle,
        raffle_winner_registry: &'a [RaffleWinner],
        is_needed_to_transfer_prize: &'a mut bool,
        is_needed_to_create_prize_transfer_proof: &'a mut bool,
        prize_received_raffle_winner_discord_user_id_hash_set: &'a mut HashSet<String>,
        prize_transfer_proof_registry: &'a mut Vec<PrizeTransferProof>,
    ) -> Result<(), Auditor<Error>> {
        let postgresql_pooled_connection = match shared_data.postgresql_connection_pool.as_ref().get().await {
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

        if *is_needed_to_transfer_prize {
            let sender_hd_wallet_2 = match PostgresqlRepository::<SenderHdWallet_2>::get(
                client,
                By_10 {
                    sender_hd_wallet_id: shared_data.sender_hd_wallet_id,
                },
            )
            .await
            {
                Ok(sender_hd_wallet_2_) => sender_hd_wallet_2_,
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

            let http_provider = match Provider::<EthersProviderHttp>::try_from(shared_data.environment_configuration.as_ref().remote_service.haqq.evm_node.url.as_str()) {
                Ok(http_provider_) => http_provider_,
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

            let chain_id = match http_provider.get_chainid().await {
                Ok(chain_id_) => chain_id_,
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

            let chain_id_ = if chain_id <= (U256::from(u64::MAX)) {
                chain_id.as_u64()
            } else {
                return Err(
                    Auditor::<Error>::new(
                        Error::create_overflow_occured(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            };

            let sender_hd_wallet_mnemonic_derevation_path_index = match u32::try_from(sender_hd_wallet_2.mnemonic_derevation_path_index) {
                Ok(sender_hd_wallet_mnemonic_derevation_path_index_) => sender_hd_wallet_mnemonic_derevation_path_index_,
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
                .phrase(sender_hd_wallet_2.mnemonic_phrase.as_str())
                .index(sender_hd_wallet_mnemonic_derevation_path_index) {
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

            let sender_wallet = match mnemonic_builder.build() {
                Ok(sender_wallet_) => sender_wallet_.with_chain_id(chain_id_),
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

            let sender_address = sender_wallet.address();

            let signer_http_provider = SignerMiddleware::new(http_provider, sender_wallet);

            let raffle_islm_prize_amount = match u64::try_from(raffle.islm_prize_amount) {
                Ok(raffle_islm_prize_amount_) => raffle_islm_prize_amount_,
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

            let aislm_amount_to_transfer = U256::from(raffle_islm_prize_amount) * U256::from(10 as u8).pow(U256::from(ISLM_COIN_PRECISION));

            'a: for raffle_winner in raffle_winner_registry.iter() {
                if prize_received_raffle_winner_discord_user_id_hash_set.contains(&raffle_winner.discord_user_id) {
                    continue 'a;
                }

                let nonce = match signer_http_provider.get_transaction_count(sender_address, None).await {
                    Ok(nonce_) => nonce_,
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

                let recipient_address = match Encoder::<Bech32>::decode(raffle_winner.bech32_address.as_str()) {
                    Ok(recipient_address_) => recipient_address_,
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

                let recipient_address_ = match recipient_address.parse::<Address>() {
                    Ok(recipient_address__) => recipient_address__,
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

                let transaction_request = TransactionRequest::new()
                    .to(recipient_address_)
                    .value(aislm_amount_to_transfer)
                    .from(sender_address)
                    .nonce(nonce);

                let pending_transaction = match signer_http_provider.send_transaction(transaction_request, None).await  {
                    Ok(pending_transaction_) => pending_transaction_.confirmations(3).retries(10),
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

                let transaction_receipt = match pending_transaction.await  {
                    Ok(transaction_receipt_) => transaction_receipt_,
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

                let transaction_receipt_ = match transaction_receipt {
                    Some(transaction_receipt__) => transaction_receipt__,
                    None => {
                        return Err(
                            Auditor::<Error>::new(
                                Error::Runtime {
                                    runtime: Runtime::Other {
                                        other: Other::new_("Transaction removed from the mempool.".into()),
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

                // TODO TODO TODO проыерить статус транзакции?
                // TODO TODO TODO проыерить статус транзакции?
                // TODO TODO TODO проыерить статус транзакции?
                // TODO TODO TODO проыерить статус транзакции?
                // TODO TODO TODO проыерить статус транзакции?
                // TODO TODO TODO проыерить статус транзакции?
                // TODO TODO TODO проыерить статус транзакции?
                // TODO TODO TODO проыерить статус транзакции?

                prize_received_raffle_winner_discord_user_id_hash_set.insert(raffle_winner.discord_user_id.clone());

                prize_transfer_proof_registry.push(
                    PrizeTransferProof::new(
                        raffle.id,
                        raffle_winner.discord_user_id.clone(),
                        transaction_receipt_.transaction_hash.encode_hex_with_prefix(),
                        Resolver::<UTCDateTime>::get_now(),
                    )
                );
            }

            *is_needed_to_transfer_prize = false;
        }

        if *is_needed_to_create_prize_transfer_proof {
            if let Err(mut error_auditor) = PostgresqlRepository::<PrizeTransferProof>::batch_insert(
                client,
                prize_transfer_proof_registry.as_slice(),
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

            *is_needed_to_create_prize_transfer_proof = false;
        }

        let mut message = "The raffle is complited.\nWinners:".to_string();

        '_a: for prize_transfer_proof in prize_transfer_proof_registry.as_slice().iter() {
            message = format!(
                "{}\n<@{}> https://explorer.haqq.network/tx/{}",
                message.as_str(),
                prize_transfer_proof.discord_user_id.as_str(),
                prize_transfer_proof.evm_transaction_hash.as_str(),
            );
        }

        let create_message = CreateMessage::new().content(message);

        // TODO количпество символов, если много победителей.
        // TODO количпество символов, если много победителей.
        // TODO количпество символов, если много победителей.
        // TODO количпество символов, если много победителей.
        // TODO количпество символов, если много победителей.
        // TODO количпество символов, если много победителей.
        // TODO количпество символов, если много победителей.
        // TODO количпество символов, если много победителей.



        if let Err(error) = shared_data
            .discord_guild_stakers_club_channel_id
            .send_message(
                http,
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

        return Ok(());
    }

    fn choose_raffle_winners<'a>(
        aislm_stake_aggregated_data: &'a AislmStakeAggregatedData,
        raffle: &'a Raffle,
    ) -> Result<Vec<RaffleWinner>, Auditor<Error>> {
        let weight_factor = match U256::try_from(Self::WEIGHT_FACTOR) {
            Ok(weight_factor_) => weight_factor_,
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

        let mut discord_user_id_stake_quantity_registry: Vec<(
            &'_ str,
            U256,
        )> = vec![];

        let mut discord_user_id_bech32_address_biggest_aislm_stake_quantity_hash_map = HashMap::<&'_ str, &'_ str>::new();

        let mut aislm_stake_for_all_users_quantity = U256::zero();

        let mut counter = U256::zero();

        '_a: for (discord_user_id, aislm_stake_data) in aislm_stake_aggregated_data.0.iter() {
            let mut aislm_stake_for_one_user_quantity = U256::zero();

            '_b: for (bech32_address, aislm_stake_quantity) in aislm_stake_data.iter() {
                aislm_stake_for_one_user_quantity += *aislm_stake_quantity;

                aislm_stake_for_all_users_quantity += *aislm_stake_quantity;

                if counter < *aislm_stake_quantity {
                    counter = *aislm_stake_quantity;

                    discord_user_id_bech32_address_biggest_aislm_stake_quantity_hash_map.insert(
                        discord_user_id.as_str(),
                        bech32_address.as_str(),
                    );
                }
            }

            counter = U256::zero();

            discord_user_id_stake_quantity_registry.push(
                (
                    discord_user_id.as_str(),
                    aislm_stake_for_one_user_quantity,
                ),
            );
        }

        let mut tikets_quantity_for_all_users = 0 as u64;

        let mut tiket_hash_map = HashMap::<u64, &'_ str>::new();

        let mut counter_panding = 0 as u64;

        let mut counter_ = 1 as u64;

        '_a: for (discord_user_id, aislm_stake_for_one_user_quantity) in discord_user_id_stake_quantity_registry.iter() {
            let tikets_quantity_for_one_user = *aislm_stake_for_one_user_quantity * weight_factor / aislm_stake_for_all_users_quantity;

            if !tikets_quantity_for_one_user.is_zero() {
                let tikets_quantity_for_one_user_ = tikets_quantity_for_one_user.as_u64();

                tikets_quantity_for_all_users += tikets_quantity_for_one_user_;

                'b: loop {
                    if counter_ <= tikets_quantity_for_one_user_ {
                        let key = counter_ + counter_panding;

                        tiket_hash_map.insert(
                            key,
                            *discord_user_id,
                        );

                        counter_ += 1;
                    } else {
                        counter_panding += counter_ - 1;

                        counter_ = 1;

                        break 'b;
                    }
                }
            }
        }

        let raffle_winner_created_at = Resolver::<UTCDateTime>::get_now();

        let mut raffle_winner_hash_map = HashMap::<&'_ str, RaffleWinner>::new();

        // TODO TODO TODO
        // TODO TODO TODO
        // TODO TODO TODO
        // TODO TODO TODO генерератор должен генерировать с сиида.
        // TODO TODO TODO
        // TODO TODO TODO
        // TODO TODO TODO
        // TODO TODO TODO генерератор должен генерировать с сиида.
        // TODO TODO TODO
        // TODO TODO TODO
        // TODO TODO TODO
        // TODO TODO TODO генерератор должен генерировать с сиида.

        '_a: for _ in 1..=raffle.winners_number {
            let raffle_winner_discord_user_id = match tiket_hash_map.get(&rand::thread_rng().gen_range(1..=tikets_quantity_for_all_users)) {
                Some(raffle_winner_discord_user_id_) => *raffle_winner_discord_user_id_,
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

            let raffle_winner_bech32_address = match discord_user_id_bech32_address_biggest_aislm_stake_quantity_hash_map.remove(raffle_winner_discord_user_id) {
                Some(raffle_winner_bech32_address_) => raffle_winner_bech32_address_,
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

            if !raffle_winner_hash_map.contains_key(&raffle_winner_discord_user_id) {
                raffle_winner_hash_map.insert(
                    raffle_winner_discord_user_id,
                    RaffleWinner::new(
                        raffle.id,
                        raffle_winner_discord_user_id.to_string(),
                        raffle_winner_bech32_address.to_string(),
                        raffle_winner_created_at,
                    ),
                );
            }
        }

        let mut raffle_winner_registry: Vec<RaffleWinner> = vec![];

        '_a: for (_, raffle_winner) in raffle_winner_hash_map.into_iter() {
            raffle_winner_registry.push(raffle_winner);
        }

        return Ok(raffle_winner_registry);
    }

    pub async fn abort_all_processes() -> () {
        let join_handle_registry = [
            &SERVE_RAFFLE_TASK_ABORT_HANDLE,
            &UPDATE_STAKE_TASK_ABORT_HANDLE,
            &FINISH_RAFFLE_TASK_ABORT_HANDLE,
        ];

        '_a: for join_handle in join_handle_registry.into_iter() {
            if let Some(ref join_handle_) = *join_handle.lock().await {
                join_handle_.abort();
            }
        }

        '_a: for join_handle in join_handle_registry.into_iter() {
            'b: loop {
                let is_task_finished = {
                    let mutex_guard = join_handle.lock().await;

                    let join_handle_ = match *mutex_guard {
                        Some(ref join_handle__) => join_handle__,
                        None => {
                            break 'b;
                        }
                    };

                    join_handle_.is_finished()
                };

                if is_task_finished {
                    *join_handle.lock().await = None;

                    break 'b;
                } else {
                    sleep(Duration::from_millis(100)).await;

                    continue 'b;
                }
            }
        }

        return ();
    }
}

struct AislmStakeAggregatedData(
    Vec<(
        String,
        Vec<(
            String,
            U256,
        )>,
    )>,
);

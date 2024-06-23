use super::run_bot::SharedData;
use super::Processor;
use crate::domain_layer::data::entity::discord_user_role::DiscordUserRole_4;
use crate::domain_layer::data::entity::verified_bech32_address::VerifiedBech32Address_3;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::control_type::Cosmos;
use crate::infrastructure_layer::data::control_type::TokioNonBlockingTask;
use crate::infrastructure_layer::data::control_type::UTCDateTime;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_7;
use crate::infrastructure_layer::functionality::repository::postgresql::update::Update_3;
use crate::infrastructure_layer::functionality::repository::postgresql::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::http_request_resolver::HttpRequestResolver;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use crate::infrastructure_layer::functionality::service::spawner::Spawner;
use chrono::Utc;
use cron::Schedule;
use ethers::types::U256;
use serenity::all::UserId;
use serenity::http::Http;
use std::clone::Clone;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio::time::Duration;

pub use crate::infrastructure_layer::data::control_type::UpdateDiscordRoles;

pub static IS_UPDATE_DISCORD_ROLES_PROCESS_EXIST: Mutex<bool> = Mutex::const_new(false);

impl Processor<UpdateDiscordRoles> {
    pub async fn process(shared_data: Arc<SharedData>) -> Result<(), Auditor<Error>> {
        {
            let mut mutex_guard = IS_UPDATE_DISCORD_ROLES_PROCESS_EXIST.lock().await;

            if *mutex_guard {
                return Ok(());
            } else {
                *mutex_guard = true;
            }
        }

        let http = Arc::new(Resolver::<Http>::create(shared_data.as_ref().environment_configuration.as_ref()));

        '_a: loop {
            if let Err(mut error_auditor) = Self::process_1(
                http.clone(),
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

                sleep(Duration::from_secs(5)).await;
            }
        }

        return Ok(());
    }

    async fn process_1(
        http: Arc<Http>,
        shared_data: Arc<SharedData>,
    ) -> Result<(), Auditor<Error>> {
        let schedule = match Schedule::from_str(
            shared_data
                .as_ref()
                .environment_configuration
                .as_ref()
                .noncontext_parameters
                .discord_roles_updating_cron_configuration
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

        '_a: for sheduled_date_time in schedule.upcoming(Utc) {
            let sheduled_ = sheduled_date_time.timestamp();

            '_b: while Resolver::<UTCDateTime>::get_now() < sheduled_ {
                sleep(Duration::from_secs(1)).await;
            }

            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                Self::process_2(
                    http.clone(),
                    shared_data.clone(),
                ),
            );
        }

        return Ok(());
    }

    async fn process_2(
        http: Arc<Http>,
        shared_data: Arc<SharedData>,
    ) -> Result<(), Auditor<Error>> {
        let mut counter = 0 as usize;

        'a: loop {
            if let Err(mut error_auditor) = Self::process_3(
                http.clone(),
                shared_data.clone(),
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

    async fn process_3(
        http: Arc<Http>,
        shared_data: Arc<SharedData>,
    ) -> Result<(), Auditor<Error>> {
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

            let registry = match PostgresqlRepository::<VerifiedBech32Address_3>::get_all_available_without_role(
                &*postgresql_pooled_connection,
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

        '_a: for registry in registry_registry.into_iter() {
            '_b: for (verified_bech32_address_discord_user_id, verified_bech32_address_value_registry) in registry.into_iter() {
                let mut aislm_stake_for_all_addresses_quantity = U256::zero();

                '_c: for verified_bech32_address_value in verified_bech32_address_value_registry.into_iter() {
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

                        aislm_stake_for_all_addresses_quantity += aislm_stake_quantity;
                    }
                }

                if aislm_stake_for_all_addresses_quantity >= shared_data_.stake_treshold_quantity_for_stakers_club_role {
                    let discord_user_id = match UserId::from_str(verified_bech32_address_discord_user_id.as_str()) {
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

                    Spawner::<TokioNonBlockingTask>::spawn_into_background(
                        Self::add_role_stakers_club_member(
                            http.clone(),
                            shared_data.clone(),
                            discord_user_id,
                        ),
                    );
                }
            }
        }

        return Ok(());
    }

    async fn add_role_stakers_club_member(
        http: Arc<Http>,
        shared_data: Arc<SharedData>,
        discord_user_id: UserId,
    ) -> Result<(), Auditor<Error>> {
        let mut counter = 0 as usize;

        'a: loop {
            if let Err(mut error_auditor) = Self::add_role_stakers_club_member_(
                http.clone(),
                shared_data.clone(),
                discord_user_id,
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
                    sleep(Duration::from_secs(5)).await;

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

    async fn add_role_stakers_club_member_(
        http: Arc<Http>,
        shared_data: Arc<SharedData>,
        discord_user_id: UserId,
    ) -> Result<(), Auditor<Error>> {
        let shared_data_ = shared_data.as_ref();

        if let Err(error) = http
            .as_ref()
            .add_member_role(
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

        if let Err(mut error_auditor) = PostgresqlRepository::<DiscordUserRole_4>::update(
            &*postgresql_pooled_connection,
            Update_3 {
                discord_user_role_stakers_club_member: true,
                discord_user_role_updated_at: Resolver::<UTCDateTime>::get_now(),
            },
            &By_7 {
                discord_user_role_discord_user_id: discord_user_id.to_string().as_str(),
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

        return Ok(());
    }
}

use crate::domain_layer::data::entity::raffle::Raffle;
use crate::domain_layer::data::entity::raffle::Raffle_5;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::control_type::DiscordCompositeCustomId;
use crate::infrastructure_layer::data::control_type::RunBot;
use crate::infrastructure_layer::data::control_type::ServeRaffle;
use crate::infrastructure_layer::data::control_type::TokioNonBlockingTask;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_8;
use crate::infrastructure_layer::functionality::repository::postgresql::PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::postgresql::update::Update_5;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use crate::infrastructure_layer::functionality::service::spawner::Spawner;
use serenity::all::CreateButton;
use serenity::builder::CreateMessage;
use serenity::http::Http;
use std::clone::Clone;
use std::sync::Arc;
use super::Processor;
use super::run_bot::SharedData;
use tokio::time::Duration;
use tokio::time::sleep;

pub use crate::infrastructure_layer::data::control_type::UpdateRaffle;

impl Processor<UpdateRaffle> {
    pub async fn process(
        shared_data: Arc<SharedData>,
        http: Arc<Http>,
        raffle_id: i64,
        update_5: Update_5,
    ) -> Result<(), Auditor<Error>> {
        Processor::<ServeRaffle>::abort_all_processes().await;

        let mut is_need_to_update = true;

        'a: loop {
            if let Err(mut error_auditor) = Self::process_1(
                http.clone(),
                shared_data.clone(),
                raffle_id,
                update_5,
                &mut is_need_to_update,
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

    async fn process_1<'a>(
        http: Arc<Http>,
        shared_data: Arc<SharedData>,
        raffle_id: i64,
        update_5: Update_5,
        is_need_to_update: &'a mut bool,
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

        let client = &*postgresql_pooled_connection;

        let by_8 = By_8 {
            raffle_id,
        };

        if *is_need_to_update {
            if let Err(mut error_auditor) = PostgresqlRepository::<Raffle_5>::update(
                client,
                update_5,
                by_8,
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

            *is_need_to_update = false;
        }

        let raffle = match PostgresqlRepository::<Raffle>::find(
            client,
            by_8,
        ).await {
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

        let raffle_ = match raffle {
            Some(raffle__) => raffle__,
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

        let message = format!(
            "The raffle updated:\nId: {}\nPrize: {} ISLM\nWinners number: {}\nWill complited at: {}",
            raffle_.id, raffle_.islm_prize_amount, raffle_.winners_number, raffle_.expired_at,
        );

        let create_button = CreateButton::new(
            Resolver::<DiscordCompositeCustomId>::from_raffle_id_to_id(
                Processor::<RunBot>::CUSTOM_ID_3,
                raffle_.id,
            ),
        )
        .label("Participate");

        let create_message = CreateMessage::new().content(message).button(create_button);

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

        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            Processor::<ServeRaffle>::process(
                shared_data.clone(),
                http,
                Arc::new(raffle_),
            ),
        );

        return Ok(());
    }
}

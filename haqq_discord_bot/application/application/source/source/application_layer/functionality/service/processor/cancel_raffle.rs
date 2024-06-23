use crate::domain_layer::data::entity::raffle::Raffle_2;
use crate::domain_layer::data::entity::raffle::Raffle_4;
use crate::domain_layer::data::entity::raffle::Raffle_Status;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::data::control_type::ServeRaffle;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By_8;
use crate::infrastructure_layer::functionality::repository::postgresql::PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::postgresql::update::Update_4;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use serenity::all::CreateMessage;
use serenity::http::Http;
use std::clone::Clone;
use std::sync::Arc;
use super::Processor;
use super::run_bot::SharedData;
use tokio::time::Duration;
use tokio::time::sleep;

pub use crate::infrastructure_layer::data::control_type::CancelRaffle;

impl Processor<CancelRaffle> {
    pub async fn process(
        shared_data: Arc<SharedData>,
        http: Arc<Http>,
        raffle_4: Raffle_4,
    ) -> Result<(), Auditor<Error>> {
        Processor::<ServeRaffle>::abort_all_processes().await;

        let mut is_need_to_update = true;

        'a: loop {
            if let Err(mut error_auditor) = Self::process_1(
                http.clone(),
                shared_data.clone(),
                raffle_4,
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
        raffle_4: Raffle_4,
        is_need_to_update: &'a mut bool,
    ) -> Result<(), Auditor<Error>> {
        if *is_need_to_update {
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

            if let Err(mut error_auditor) = PostgresqlRepository::<Raffle_2>::update(
                &*postgresql_pooled_connection,
                Update_4 {
                    raffle_status: Raffle_Status::Canceled,
                },
                By_8 {
                    raffle_id: raffle_4.id,
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

            *is_need_to_update = false;
        }

        let message = "The raffle is canceled.";

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
}

use crate::domain_layer::data::entity::raffle::Raffle;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::control_type::ServeRaffle;
use crate::infrastructure_layer::data::control_type::UTCDateTime;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use serenity::http::Http;
use std::sync::Arc;
use super::Processor;
use super::run_bot::SharedData;

pub use crate::infrastructure_layer::data::control_type::CompleteRaffle;

impl Processor<CompleteRaffle> {
    pub async fn process(
        shared_data: Arc<SharedData>,
        http: Arc<Http>,
        raffle: Arc<Raffle>,
    ) -> Result<(), Auditor<Error>> {
        Processor::<ServeRaffle>::abort_all_processes().await;

        let _ = Processor::<ServeRaffle>::finish_raffle_and_notify(
            http,
            shared_data,
            raffle,
            Resolver::<UTCDateTime>::get_now(),
        ).await;

        return Ok(());
    }
}

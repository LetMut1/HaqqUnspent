pub use crate::application_layer::functionality::action_processor::balance_snapshot::get_history::BalanceSnapshot___GetHistory;
use crate::{
    application_layer::functionality::{
        action_processor::ActionProcessor,
        service::processor::{
            generalized_action::GeneralizedAction,
            Processor,
        },
    },
    infrastructure_layer::{
        data::control_type::{
            HttpBodyData,
            Response,
        },
        functionality::service::serializer::json::Json,
    },
    presentation_layer::functionality::{
        action::Action,
        service::extractor::Extractor,
    },
};
use clickhouse::Client;
use http::request::Parts;
use hyper::Body;
use matchit::Params;

impl Action<BalanceSnapshot___GetHistory> {
    pub async fn run<'a>(body: &'a mut Body, parts: &'a Parts, route_parameters: &'a Params<'_, '_>, clickhouse_client: Client) -> Response {
        return Processor::<GeneralizedAction>::process::<_, _, _, _, _, _, _, Json>(
            body,
            parts,
            route_parameters,
            clickhouse_client,
            Extractor::<HttpBodyData>::extract::<_, Json>,
            ActionProcessor::<BalanceSnapshot___GetHistory>::process,
        )
        .await;
    }
}

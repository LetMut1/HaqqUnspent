pub use crate::infrastructure_layer::data::control_type::Task___HealthCheck;
use crate::{
    application_layer::functionality::{
        action_processor::ActionProcessor,
        service::processor::{
            generalized_action::GeneralizedAction,
            Processor,
        },
    },
    infrastructure_layer::{
        data::{
            control_type::{
                HttpBodyData,
                Response,
            },
            environment_configuration::EnvironmentConfiguration,
        },
        functionality::service::serializer::json::Json,
    },
    presentation_layer::functionality::{
        action::Action,
        service::extractor::Extractor,
    },
};
use http::request::Parts;
use hyper::Body;
use matchit::Params;
use std::sync::Arc;

impl Action<Task___HealthCheck> {
    pub async fn run<'a>(body: &'a mut Body, parts: &'a Parts, route_parameters: &'a Params<'_, '_>, environment_configuration: Arc<EnvironmentConfiguration>) -> Response {
        return Processor::<GeneralizedAction>::process_option::<_, _, _, _, _, _, Json>(
            body,
            parts,
            route_parameters,
            environment_configuration,
            Extractor::<HttpBodyData>::extract::<_, Json>,
            ActionProcessor::<Task___HealthCheck>::process,
        )
        .await;
    }
}

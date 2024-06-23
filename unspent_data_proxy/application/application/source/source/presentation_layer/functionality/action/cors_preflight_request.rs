pub use crate::application_layer::functionality::action_processor::cors_preflight_request::CorsPreflightRequest;
use crate::{
    application_layer::functionality::{
        action_processor::ActionProcessor,
        service::reactor::Reactor,
    },
    infrastructure_layer::data::control_type::{
        ActionRound,
        Response,
    },
    presentation_layer::functionality::action::Action,
};
use http::request::Parts;

impl Action<CorsPreflightRequest> {
    pub fn run<'a>(parts: &'a Parts) -> Response {
        let response = ActionProcessor::<CorsPreflightRequest>::process();

        Reactor::<(
            ActionRound,
            Response,
        )>::react(
            parts,
            &response,
        );

        return response;
    }
}

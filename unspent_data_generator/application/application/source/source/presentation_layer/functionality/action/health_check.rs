pub use crate::application_layer::functionality::action_processor::health_check::HealthCheck;
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

impl Action<HealthCheck> {
    pub fn run<'a>(parts: &'a Parts) -> Response {
        let response = ActionProcessor::<HealthCheck>::process();

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

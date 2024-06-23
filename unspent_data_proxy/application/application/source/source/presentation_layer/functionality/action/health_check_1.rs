pub use crate::application_layer::functionality::action_processor::health_check_1::HealthCheck1;
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

impl Action<HealthCheck1> {
    pub fn run<'a>(parts: &'a Parts) -> Response {
        let response = ActionProcessor::<HealthCheck1>::process();

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

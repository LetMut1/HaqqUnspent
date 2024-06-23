use crate::application_layer::functionality::action_processor::ActionProcessor;
use crate::application_layer::functionality::service::reactor::Reactor;
use crate::infrastructure_layer::data::control_type::ActionRound;
use crate::infrastructure_layer::data::control_type::Response;
use crate::presentation_layer::functionality::action::Action;
use http::request::Parts;

pub use crate::application_layer::functionality::action_processor::health_check::HealthCheck;

impl Action<HealthCheck> {
    pub fn run<'a>(parts: &'a Parts) -> Response {
        let response = ActionProcessor::<HealthCheck>::process();

        Reactor::<(
            ActionRound,
            Response,
        )>::react(
            parts, &response,
        );

        return response;
    }
}

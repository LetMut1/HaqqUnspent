pub use crate::application_layer::functionality::action_processor::route_not_found::RouteNotFound;
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

impl Action<RouteNotFound> {
    pub fn run<'a>(parts: &'a Parts) -> Response {
        let response = ActionProcessor::<RouteNotFound>::process();

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

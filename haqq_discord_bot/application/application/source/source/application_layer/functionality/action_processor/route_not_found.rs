use crate::application_layer::functionality::action_processor::ActionProcessor;
use crate::infrastructure_layer::data::control_type::Response;
use crate::infrastructure_layer::functionality::service::creator::Creator;

pub use crate::infrastructure_layer::data::control_type::RouteNotFound;

impl ActionProcessor<RouteNotFound> {
    pub fn process() -> Response {
        return Creator::<Response>::create_not_found();
    }
}

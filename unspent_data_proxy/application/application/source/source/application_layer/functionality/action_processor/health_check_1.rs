pub use crate::infrastructure_layer::data::control_type::HealthCheck1;
use crate::{
    application_layer::functionality::action_processor::ActionProcessor,
    infrastructure_layer::{
        data::control_type::Response,
        functionality::service::creator::Creator,
    },
};

impl ActionProcessor<HealthCheck1> {
    pub fn process() -> Response {
        return Creator::<Response>::create_ok(None);
    }
}

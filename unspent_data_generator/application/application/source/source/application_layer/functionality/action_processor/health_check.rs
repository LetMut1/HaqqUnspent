pub use crate::infrastructure_layer::data::control_type::HealthCheck;
use crate::{
    application_layer::functionality::action_processor::ActionProcessor,
    infrastructure_layer::{
        data::control_type::Response,
        functionality::service::creator::Creator,
    },
};

impl ActionProcessor<HealthCheck> {
    pub fn process() -> Response {
        return Creator::<Response>::create_ok_();
    }
}

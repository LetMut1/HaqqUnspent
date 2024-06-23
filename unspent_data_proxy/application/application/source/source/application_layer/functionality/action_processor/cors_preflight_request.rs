pub use crate::infrastructure_layer::data::control_type::CorsPreflightRequest;
use crate::{
    application_layer::functionality::action_processor::ActionProcessor,
    infrastructure_layer::{
        data::control_type::Response,
        functionality::service::creator::Creator,
    },
};

impl ActionProcessor<CorsPreflightRequest> {
    pub fn process() -> Response {
        return Creator::<Response>::create_cors_ok();
    }
}

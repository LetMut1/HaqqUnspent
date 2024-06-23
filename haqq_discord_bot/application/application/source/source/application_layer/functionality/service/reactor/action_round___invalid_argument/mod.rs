use super::Reactor;
use crate::infrastructure_layer::data::control_type::TokioNonBlockingTask;
use crate::infrastructure_layer::functionality::service::creator::response::Response;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use crate::infrastructure_layer::functionality::service::spawner::Spawner;
use http::request::Parts;

pub use crate::infrastructure_layer::data::auditor::Auditor;
pub use crate::infrastructure_layer::data::control_type::ActionRound;
pub use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;

impl
    Reactor<(
        ActionRound,
        Auditor<InvalidArgument>,
    )>
{
    pub fn react<'a>(
        request_parts: &'a Parts,
        response: &'a Response,
        invalid_argument_auditor: Auditor<InvalidArgument>,
    ) -> () {
        let request_uri = request_parts.uri.path().to_string();

        let request_method = request_parts.method.to_string();

        let response_status_code = response.status().as_u16();

        let future = async move {
            Logger::<(
                ActionRound,
                Auditor<InvalidArgument>,
            )>::log(
                request_uri.as_str(),
                request_method.as_str(),
                response_status_code,
                &invalid_argument_auditor,
            );

            return Ok(());
        };

        Spawner::<TokioNonBlockingTask>::spawn_into_background(future);

        return ();
    }
}

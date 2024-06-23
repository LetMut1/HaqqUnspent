use super::Logger;
use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use tracing::info;

pub use crate::infrastructure_layer::data::control_type::ActionRound;
pub use crate::infrastructure_layer::data::control_type::Response;

impl
    Logger<(
        ActionRound,
        Response,
    )>
{
    pub fn log<'a>(
        request_uri: &'a str,
        request_method: &'a str,
        response_status_code: u16,
    ) -> () {
        let message = Formatter::<ActionRound>::format(
            request_uri,
            request_method,
            response_status_code,
            None,
        );

        info!(
            "{}",
            message.as_str()
        );

        return ();
    }
}

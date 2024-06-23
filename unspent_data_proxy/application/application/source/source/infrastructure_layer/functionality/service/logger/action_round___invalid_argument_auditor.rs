use super::Logger;
pub use crate::infrastructure_layer::data::{
    auditor::Auditor,
    control_type::ActionRound,
    invalid_argument::InvalidArgument,
};
use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use tracing::info;

impl
    Logger<(
        ActionRound,
        Auditor<InvalidArgument>,
    )>
{
    pub fn log<'a>(request_uri: &'a str, request_method: &'a str, response_status_code: u16, invalid_argument_auditor: &'a Auditor<InvalidArgument>) -> () {
        let message = Formatter::<Auditor<InvalidArgument>>::format(&invalid_argument_auditor);

        let message_ = Formatter::<ActionRound>::format(
            request_uri,
            request_method,
            response_status_code,
            Some(message.as_str()),
        );

        info!(
            "{}",
            message_.as_str()
        );

        return ();
    }
}

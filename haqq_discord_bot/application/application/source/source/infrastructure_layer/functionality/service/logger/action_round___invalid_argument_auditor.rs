use super::Logger;
use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use tracing::info;

pub use crate::infrastructure_layer::data::auditor::Auditor;
pub use crate::infrastructure_layer::data::control_type::ActionRound;
pub use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;

impl
    Logger<(
        ActionRound,
        Auditor<InvalidArgument>,
    )>
{
    pub fn log<'a>(
        request_uri: &'a str,
        request_method: &'a str,
        response_status_code: u16,
        invalid_argument_auditor: &'a Auditor<InvalidArgument>,
    ) -> () {
        let message = Formatter::<ActionRound>::format(
            request_uri,
            request_method,
            response_status_code,
            Some(Formatter::<Auditor<InvalidArgument>>::format(&invalid_argument_auditor).as_str()),
        );

        info!(
            "{}",
            message.as_str()
        );

        return ();
    }
}

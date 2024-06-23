use super::Logger;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use tracing::error;

pub use crate::infrastructure_layer::data::control_type::ActionRound;
pub use crate::infrastructure_layer::data::error::Error;

impl
    Logger<(
        ActionRound,
        Auditor<Error>,
    )>
{
    pub fn log<'a>(
        request_uri: &'a str,
        request_method: &'a str,
        response_status_code: u16,
        error_auditor: &'a Auditor<Error>,
    ) -> () {
        let message = Formatter::<ActionRound>::format(
            request_uri,
            request_method,
            response_status_code,
            Some(Formatter::<Auditor<Error>>::format(error_auditor).as_str()),
        );

        error!(
            "{}",
            message.as_str()
        );

        return ();
    }
}

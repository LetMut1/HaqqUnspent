use super::Formatter;
pub use crate::infrastructure_layer::data::control_type::ActionRound;

impl Formatter<ActionRound> {
    pub fn format<'a>(request_uri: &'a str, request_method: &'a str, response_status_code: u16, context: Option<&'a str>) -> String {
        let message = match context {
            Some(context_) => {
                format!(
                "\'{} {} {}\':\n{}",
                response_status_code, request_method, request_uri, context_,
            )
            }
            None => {
                format!(
                "\'{} {} {}\'",
                response_status_code, request_method, request_uri,
            )
            }
        };

        return message;
    }
}

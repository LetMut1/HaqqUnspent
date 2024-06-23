use super::Formatter;
use crate::infrastructure_layer::data::{
    auditor::Auditor,
    backtrace::Backtrace,
    invalid_argument::InvalidArgument,
};

impl Formatter<Auditor<InvalidArgument>> {
    pub fn format<'a>(invalid_argument_auditor: &'a Auditor<InvalidArgument>) -> String {
        let backtrace_message = Formatter::<Backtrace>::format(invalid_argument_auditor.get_backtrace());

        let invalid_argument_message = match invalid_argument_auditor.get_subject().context {
            Some(ref context) => {
                format!(
                    "InvalidArgument ({})\n{}",
                    context.as_str(),
                    backtrace_message.as_str(),
                )
            }
            None => {
                format!(
                    "InvalidArgument:\n{}",
                    backtrace_message.as_str(),
                )
            }
        };

        return invalid_argument_message;
    }
}

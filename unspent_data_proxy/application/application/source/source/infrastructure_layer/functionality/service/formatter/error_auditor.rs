use super::Formatter;
use crate::infrastructure_layer::data::{
    auditor::Auditor,
    backtrace::Backtrace,
    error::{
        Error,
        Resource,
        Runtime,
    },
};

impl Formatter<Auditor<Error>> {
    pub fn format<'a>(error_auditor: &'a Auditor<Error>) -> String {
        let backtrace_message = Formatter::<Backtrace>::format(error_auditor.get_backtrace());

        let error_message = match *error_auditor.get_subject() {
            Error::Logic {
                message,
            } => {
                format!(
                    "LogicError: {}.",
                    message
                )
            }
            Error::Runtime {
                runtime: ref run_time_error,
            } => {
                match *run_time_error {
                    Runtime::Other {
                        ref other,
                    } => {
                        format!(
                            "OtherRuntimeError: {}.",
                            other.get_error()
                        )
                    }
                    Runtime::Resource {
                        ref resource,
                    } => {
                        match *resource {
                            Resource::Clickhouse => "ClickhouseResourceRuntimeError.".to_string(),
                        }
                    }
                }
            }
        };

        return format!(
            "{}:\n{}",
            error_message.as_str(),
            backtrace_message.as_str(),
        );
    }
}

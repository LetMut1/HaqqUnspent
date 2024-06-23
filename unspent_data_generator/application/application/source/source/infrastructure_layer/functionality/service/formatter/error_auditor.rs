use super::Formatter;
use crate::infrastructure_layer::data::{
    auditor::Auditor,
    backtrace::Backtrace,
    error::{
        Error,
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

impl<'a> Formatter<&'a [Auditor<Error>]> {
    pub fn format(error_auditor_registry: &'a [Auditor<Error>]) -> String {
        let mut error_message = String::new();

        '_a: for (index, error_auditor) in error_auditor_registry.iter().enumerate() {
            if error_auditor_registry.len() == 1 {
                error_message = format!(
                    "Previous errors: \n[\n{}\n]",
                    Formatter::<Auditor<Error>>::format(error_auditor).as_str(),
                );
            } else {
                if index == 0 {
                    error_message = format!(
                        "Previous errors: \n[\n{}",
                        Formatter::<Auditor<Error>>::format(error_auditor).as_str(),
                    );
                } else {
                    if index == (error_auditor_registry.len() - 1) {
                        error_message = format!(
                            "{}\n\n{}\n]",
                            error_message.as_str(),
                            Formatter::<Auditor<Error>>::format(error_auditor).as_str(),
                        );
                    } else {
                        error_message = format!(
                            "{}\n\n{}",
                            error_message.as_str(),
                            Formatter::<Auditor<Error>>::format(error_auditor).as_str(),
                        );
                    }
                }
            }
        }

        return error_message;
    }
}

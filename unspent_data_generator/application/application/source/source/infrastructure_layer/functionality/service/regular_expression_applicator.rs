use crate::infrastructure_layer::data::{
    auditor::Auditor,
    backtrace::BacktracePart,
    error::{
        Error,
        Other,
        Runtime,
    },
};
use regex::Regex;
use std::sync::OnceLock;

static FLOAT_NUMBER_REGEX: OnceLock<Regex> = OnceLock::new();

pub struct RegularExpressionApplicator;

impl RegularExpressionApplicator {
    pub fn is_float_number<'a>(value: &'a str) -> Result<bool, Auditor<Error>> {
        const REGEX: &'static str = r"^[+\-]?(?:(?:0|[1-9]\d*)(?:\.\d*)?|\.\d+)(?:\d[eE][+\-]?\d+)?$";

        let result = match FLOAT_NUMBER_REGEX.get() {
            Some(regex) => regex.is_match(value),
            None => {
                let regex = match Regex::new(REGEX) {
                    Ok(regex_) => regex_,
                    Err(error) => {
                        return Err(
                            Auditor::<Error>::new(
                                Error::Runtime {
                                    runtime: Runtime::Other {
                                        other: Other::new(error),
                                    },
                                },
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                ),
                            ),
                        )
                    }
                };

                let result_ = regex.is_match(value);

                if let Err(_) = FLOAT_NUMBER_REGEX.set(regex) {
                    return Err(
                        Auditor::<Error>::new(
                            Error::Runtime {
                                runtime: Runtime::Other {
                                    other: Other::new_("Setter access race - this is a very-very rare but possible case in this code architecture.".into()),
                                },
                            },
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }

                result_
            }
        };

        return Ok(result);
    }
}

#[cfg(test)]
mod tests {
    use super::RegularExpressionApplicator;
    use std::error::Error;

    #[test]
    fn is_float_number() -> Result<(), Box<dyn Error + 'static>> {
        let value_registry = [
            "0",
            "+0",
            "-0",
            "0.0",
            "+0.0",
            "-0.0",
            "0.1",
            "+0.1",
            "-0.1",
            "0.123456789123456789123456789123456789123456789123456789",
            "+0.123456789123456789123456789123456789123456789123456789",
            "-0.123456789123456789123456789123456789123456789123456789",
            "123456789123456789123456789123456789123456789123456789.123456789123456789123456789123456789123456789123456789",
            "+123456789123456789123456789123456789123456789123456789.123456789123456789123456789123456789123456789123456789",
            "-123456789123456789123456789123456789123456789123456789.123456789123456789123456789123456789123456789123456789",
            ".1",
            "+.1",
            "-.1",
            "1.2e3",
            "+1.2e3",
            "-1.2e3",
        ];

        for value in value_registry.iter() {
            assert!(
                RegularExpressionApplicator::is_float_number(value)?,
                "Invalid value: {:?}",
                value,
            );
        }

        return Ok(());
    }
}

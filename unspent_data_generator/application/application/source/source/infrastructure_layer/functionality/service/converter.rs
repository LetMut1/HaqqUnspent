use crate::infrastructure_layer::data::{
    auditor::Auditor,
    backtrace::BacktracePart,
    error::{
        Error,
        Other,
        Runtime,
    },
};
use core::marker::Sized;
use std::convert::TryFrom;

pub struct Converter;

pub trait Convert<F, T>
where
    F: Sized,
    T: Sized,
{
    fn convert(subject: F) -> Result<T, Auditor<Error>>;
}

impl Convert<i64, u32> for Converter {
    fn convert(subject: i64) -> Result<u32, Auditor<Error>> {
        let converted_subject = match u32::try_from(subject) {
            Ok(converted_subject_) => converted_subject_,
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

        return Ok(converted_subject);
    }
}

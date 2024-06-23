use super::Resolver;
pub use crate::infrastructure_layer::data::control_type::UTCDateTime;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        error::Error,
    },
    functionality::service::converter::{
        Convert,
        Converter,
    },
};
use chrono::Utc;

impl Resolver<UTCDateTime> {
    pub fn get_now() -> i64 {
        return Utc::now().timestamp();
    }

    pub fn get_now_() -> Result<u32, Auditor<Error>> {
        let now = match <Converter as Convert<i64, u32>>::convert(Self::get_now()) {
            Ok(now_) => now_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        return Ok(now);
    }
}

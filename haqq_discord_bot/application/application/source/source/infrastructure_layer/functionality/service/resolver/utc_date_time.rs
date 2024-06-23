use super::Resolver;
use crate::application_layer::functionality::service::reactor::action_round___invalid_argument::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use chrono::DateTime;
use chrono::Utc;

pub use crate::infrastructure_layer::data::control_type::UTCDateTime;

impl Resolver<UTCDateTime> {
    pub fn get_now() -> i64 {
        return Utc::now().timestamp();
    }

    pub fn from_unixtime_to_timestamp(unix_time: i64) -> Result<String, Auditor<Error>> {
        let date_time = match DateTime::<Utc>::from_timestamp(
            unix_time, 0,
        ) {
            Some(date_time_) => date_time_,
            None => {
                return Err(
                    Auditor::<Error>::new(
                        Error::create_invalid_value(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        return Ok(date_time.to_rfc3339());
    }
}

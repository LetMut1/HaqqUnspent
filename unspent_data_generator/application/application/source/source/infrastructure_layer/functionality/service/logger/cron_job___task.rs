use super::Logger;
use crate::{
    domain_layer::data::entity::task::NamedTask,
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            error::Error,
        },
        functionality::service::formatter::Formatter,
    },
};
pub use crate::{
    domain_layer::data::entity::task::Task,
    infrastructure_layer::data::control_type::CronJob,
};
use chrono::{
    DateTime,
    Utc,
};
use tracing::{
    error,
    info,
};

impl<T>
    Logger<(
        CronJob,
        Task<T>,
    )>
where
    T: NamedTask,
{
    pub fn log<'a>(sheduled_date_time: &'a DateTime<Utc>, error_auditor: &'a Option<Auditor<Error>>) -> () {
        let message = Formatter::<(
            CronJob,
            Task<T>,
        )>::format(
            sheduled_date_time,
            error_auditor,
        );

        match error_auditor {
            Some(_) => {
                error!(
                    "{}",
                    message.as_str(),
                );
            }
            None => {
                info!(
                    "{}",
                    message.as_str(),
                );
            }
        }

        return ();
    }
}

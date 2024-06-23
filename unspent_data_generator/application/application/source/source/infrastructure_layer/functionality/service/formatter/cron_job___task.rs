use super::Formatter;
use crate::{
    domain_layer::data::entity::task::NamedTask,
    infrastructure_layer::data::{
        auditor::Auditor,
        error::Error,
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

impl<T>
    Formatter<(
        CronJob,
        Task<T>,
    )>
where
    T: NamedTask,
{
    pub fn format<'a>(sheduled_date_time: &'a DateTime<Utc>, error_auditor: &'a Option<Auditor<Error>>) -> String {
        let sheduled_date_time_ = sheduled_date_time.to_rfc3339();

        let message = match error_auditor {
            Some(ref error_auditror_) => {
                format!(
                    "The {} task executing at {} via cron job failed:\n{}",
                    <T as NamedTask>::get_name(),
                    sheduled_date_time_.as_str(),
                    Formatter::<Auditor<Error>>::format(error_auditror_).as_str(),
                )
            }
            None => {
                format!(
                    "The {} task executing at {} via cron job is complited successfully.",
                    <T as NamedTask>::get_name(),
                    sheduled_date_time_.as_str(),
                )
            }
        };

        return message;
    }
}

use super::Runner;
use crate::{
    domain_layer::data::entity::task::{
        NamedTask,
        Task,
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            control_type::{
                CronJob,
                TokioNonBlockingTask,
                UTCDateTime,
            },
            environment_configuration::EnvironmentConfiguration,
            error::{
                Error,
                Other,
                Runtime,
            },
        },
        functionality::service::{
            logger::Logger,
            resolver::Resolver,
            spawner::Spawner,
        },
    },
};
use chrono::Utc;
use cron::Schedule;
use std::{
    future::Future,
    str::FromStr,
    sync::Arc,
};
use tokio::time::{
    sleep,
    Duration,
};

impl<T>
    Runner<(
        CronJob,
        Task<T>,
    )>
where
    T: NamedTask,
{
    pub async fn run<'a, J, F>(cron_configuration: &'a str, environment_configuration: Arc<EnvironmentConfiguration>, job: J) -> Result<(), Auditor<Error>>
    where
        J: Fn(Arc<EnvironmentConfiguration>) -> F + Send + Sync + Copy + 'static,
        F: Future<Output = Result<(), Auditor<Error>>> + Send + 'static,
    {
        let schedule = match Schedule::from_str(cron_configuration) {
            Ok(shedule_) => shedule_,
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
                );
            }
        };

        '_a: for sheduled_date_time in schedule.upcoming(Utc) {
            let sheduled_ = sheduled_date_time.timestamp();

            '_b: while Resolver::<UTCDateTime>::get_now() < sheduled_ {
                sleep(Duration::from_millis(500)).await;
            }

            let environment_configuration_ = environment_configuration.clone();

            let future = async move {
                match job(environment_configuration_).await {
                    Ok(_) => {
                        Logger::<(
                            CronJob,
                            Task<T>,
                        )>::log(
                            &sheduled_date_time,
                            &None,
                        );
                    }
                    Err(mut error_auditor) => {
                        error_auditor.add_backtrace_part(
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        );

                        Logger::<(
                            CronJob,
                            Task<T>,
                        )>::log(
                            &sheduled_date_time,
                            &Some(error_auditor),
                        );
                    }
                };

                return Ok(());
            };

            Spawner::<TokioNonBlockingTask>::spawn_into_background(future);
        }

        return Ok(());
    }
}

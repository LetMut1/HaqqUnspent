use super::CommandProcessor;
use crate::application_layer::functionality::service::processor::Processor;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::control_type::RunServer;
use crate::infrastructure_layer::data::control_type::TokioNonBlockingTask;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::functionality::service::creator::postgresql_connection_pool_no_tls::PostgresqlConnectionPoolNoTls;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use crate::infrastructure_layer::functionality::service::spawner::Spawner;
use std::sync::Arc;
use tokio::runtime::Builder;
use tokio::select;
use tokio::signal::unix::signal;
use tokio::signal::unix::SignalKind;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio::time::Duration;
use tracing::subscriber::set_global_default;
use tracing::Level;
use tracing_appender::non_blocking::NonBlockingBuilder;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::RollingFileAppender;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::FmtSubscriber;

#[cfg(not(feature = "file_log"))]
use std::io::stdout;

pub use crate::infrastructure_layer::data::control_type::RunBot;

pub static IS_SHUTDOWN_SIGNAL_RECEIVED: Mutex<bool> = Mutex::const_new(false);

pub static PROCESS_CAN_NOT_BE_INTERRUPTED_UNTIL_COMPLETED_QUANTITY: Mutex<usize> = Mutex::const_new(0);

impl CommandProcessor<RunBot> {
    pub fn process<'a>(environment_configuration_directory_path: &'a str) -> Result<(), Auditor<Error>> {
        let environment_configuration = match Resolver::<EnvironmentConfiguration>::load_from_file(environment_configuration_directory_path) {
            Ok(environment_configuration_) => environment_configuration_,
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

        let _worker_guard = match Self::configure_logger(&environment_configuration) {
            Ok(worker_guard_) => worker_guard_,
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

        if let Err(mut error_auditor) = Self::run_runtime(environment_configuration) {
            error_auditor.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                ),
            );

            return Err(error_auditor);
        }

        return Ok(());
    }

    fn configure_logger<'a>(environment_configuration: &'a EnvironmentConfiguration) -> Result<WorkerGuard, Auditor<Error>> {
        let non_blocking;

        let worker_guard;

        let logger_level;

        #[cfg(feature = "file_log")]
        {
            let rolling_file_appender = RollingFileAppender::new(
                Rotation::DAILY,
                environment_configuration.logging.directory_path.as_str(),
                environment_configuration.logging.file_name_prefix.as_str(),
            );

            (
                non_blocking,
                worker_guard,
            ) = NonBlockingBuilder::default().finish(rolling_file_appender);
        }

        #[cfg(not(feature = "file_log"))]
        {
            (
                non_blocking,
                worker_guard,
            ) = NonBlockingBuilder::default().finish(stdout());
        }

        #[cfg(feature = "logger_level_trace")]
        {
            logger_level = Level::TRACE;
        }

        #[cfg(not(feature = "logger_level_trace"))]
        {
            logger_level = Level::INFO;
        }

        let fmt_subscriber = FmtSubscriber::builder()
            .with_max_level(logger_level)
            .with_writer(non_blocking)
            .with_file(false)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_ansi(false)
            .finish();

        if let Err(error) = set_global_default(fmt_subscriber) {
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

        return Ok(worker_guard);
    }

    fn run_runtime(environment_configuration: EnvironmentConfiguration) -> Result<(), Auditor<Error>> {
        if environment_configuration.tokio_runtime.maximum_blocking_threads_quantity == 0
            || environment_configuration.tokio_runtime.worker_threads_quantity == 0
            || environment_configuration.tokio_runtime.worker_thread_stack_size < (1024 * 1024)
        {
            return Err(
                Auditor::<Error>::new(
                    Error::Logic {
                        message: "Invalid Tokio configuration.",
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        let runtime = match Builder::new_multi_thread()
            .max_blocking_threads(environment_configuration.tokio_runtime.maximum_blocking_threads_quantity)
            .worker_threads(environment_configuration.tokio_runtime.worker_threads_quantity)
            .thread_stack_size(environment_configuration.tokio_runtime.worker_thread_stack_size)
            .enable_all()
            .build()
        {
            Ok(runtime_) => runtime_,
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

        if let Err(mut error_auditor) = runtime.block_on(Self::run_server_and_bot(environment_configuration)) {
            error_auditor.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                ),
            );

            return Err(error_auditor);
        }

        return Ok(());
    }

    async fn run_server_and_bot(environment_configuration: EnvironmentConfiguration) -> Result<(), Auditor<Error>> {
        let postgresql_connection_pool = match Creator::<PostgresqlConnectionPoolNoTls>::create(environment_configuration.resource.postgresql.configuration.as_str()).await {
            Ok(postgresql_connection_pool_) => Arc::new(postgresql_connection_pool_),
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

        let environment_configuration_ = Arc::new(environment_configuration);

        Spawner::<TokioNonBlockingTask>::spawn_into_background(Processor::<RunServer>::process(environment_configuration_.clone()));

        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            Processor::<RunBot>::process(
                environment_configuration_.clone(),
                postgresql_connection_pool.clone(),
            ),
        );

        let mut signal_interrupt = match signal(SignalKind::interrupt()) {
            Ok(signal_interrupt_) => signal_interrupt_,
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

        let mut signal_terminate = match signal(SignalKind::terminate()) {
            Ok(signal_terminate_) => signal_terminate_,
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

        let signal_interrupt_future = signal_interrupt.recv();

        let signal_terminate_future = signal_terminate.recv();

        let shutdown_signal_future = async {
            select! {
                _ = signal_interrupt_future => {
                    ()
                },
                _ = signal_terminate_future => {
                    ()
                },
            }
        };

        shutdown_signal_future.await;

        *IS_SHUTDOWN_SIGNAL_RECEIVED.lock().await = true;

        'a: loop {
            let process_can_not_be_interrupted_until_completed_quantity = { *PROCESS_CAN_NOT_BE_INTERRUPTED_UNTIL_COMPLETED_QUANTITY.lock().await };

            if process_can_not_be_interrupted_until_completed_quantity == 0 {
                break 'a;
            } else {
                sleep(Duration::from_secs(1)).await;

                continue 'a;
            }
        }

        return Ok(());
    }
}

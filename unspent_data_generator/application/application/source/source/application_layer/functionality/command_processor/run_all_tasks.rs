use super::CommandProcessor;
pub use crate::infrastructure_layer::data::control_type::RunAllTasks;
use crate::{
    domain_layer::{
        data::entity::task::{
            generate_aggregated_balance_snapshot::GenerateAggregatedBalanceSnapshot,
            generate_base_balance_snapshot::GenerateBaseBalanceSnapshot,
            update_assets___generate_asset_snapshot::UpdateAssets__GenerateAssetSnapshot,
            update_assets_for_subportfolio_trackable_wallet::UpdateAssetsForSubportfolioTrackableWallet,
            Task as Task_,
        },
        functionality::service::executor::Executor,
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            control_type::{
                CronJob,
                HealthCheck,
                Request,
                Response,
                RouteNotFound,
                Task___ForceExecute,
                Task___HealthCheck,
                TokioNonBlockingTask,
            },
            environment_configuration::EnvironmentConfiguration,
            error::{
                Error,
                Other,
                Runtime,
            },
            void::ErrorVoid,
        },
        functionality::service::{
            logger::Logger,
            resolver::Resolver,
            runner::Runner,
            spawner::Spawner,
        },
    },
    presentation_layer::{
        data::action_route::{
            matching::{
                ActionRoute,
                Task,
            },
            ACTION_ROUTE,
        },
        functionality::action::Action,
    },
};
use hyper::{
    server::conn::AddrStream,
    service::{
        make_service_fn,
        service_fn,
    },
    Method,
    Server,
};
use matchit::Router;
#[cfg(not(feature = "file_log"))]
use std::io::stdout;
use std::{
    future::Future,
    net::ToSocketAddrs,
    sync::Arc,
    time::Duration,
};
use tokio::{
    runtime::Builder,
    select,
    signal::unix::{
        signal,
        SignalKind,
    },
};
use tracing::{
    subscriber::set_global_default,
    Level,
};
use tracing_appender::{
    non_blocking::{
        NonBlockingBuilder,
        WorkerGuard,
    },
    rolling::{
        RollingFileAppender,
        Rotation,
    },
};
use tracing_subscriber::FmtSubscriber;

type Router_ = Router<ActionRoute>;

impl CommandProcessor<RunAllTasks> {
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

        if let Err(mut error_auditor) = runtime.block_on(Self::run_server_and_tasks(environment_configuration)) {
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

    fn create_router() -> Result<Router_, Auditor<Error>> {
        let mut router = Router::new();

        if let Err(error) = router.insert(
            ACTION_ROUTE.task.force_execute,
            ActionRoute::Task {
                task: Task::ForceExecute,
            },
        ) {
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

        if let Err(error) = router.insert(
            ACTION_ROUTE.task.health_check,
            ActionRoute::Task {
                task: Task::HealthCheck,
            },
        ) {
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

        if let Err(error) = router.insert(
            ACTION_ROUTE.health_check,
            ActionRoute::HealthCheck,
        ) {
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

        return Ok(router);
    }

    async fn run_server_and_tasks(environment_configuration: EnvironmentConfiguration) -> Result<(), Auditor<Error>> {
        let router = match Self::create_router() {
            Ok(router_) => Arc::new(router_),
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

        let mut application_http_socket_address_registry = match environment_configuration.application_server.tcp.socket_address.to_socket_addrs() {
            Ok(application_http_socket_address_registry_) => application_http_socket_address_registry_,
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

        let application_http_socket_address = match application_http_socket_address_registry.next() {
            Some(application_http_socket_address_) => application_http_socket_address_,
            None => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Logic {
                            message: "Invalid socket address.",
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let mut server_builder = match Server::try_bind(&application_http_socket_address) {
            Ok(builder_) => builder_,
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

        server_builder = server_builder
            .tcp_keepalive_retries(environment_configuration.application_server.tcp.keepalive.retries_quantity)
            .tcp_nodelay(environment_configuration.application_server.tcp.nodelay)
            .tcp_sleep_on_accept_errors(environment_configuration.application_server.tcp.sleep_on_accept_errors);

        server_builder = match environment_configuration.application_server.tcp.keepalive.duration {
            Some(duration) => server_builder.tcp_keepalive(Some(Duration::from_secs(duration))),
            None => server_builder.tcp_keepalive(None),
        };

        server_builder = match environment_configuration.application_server.tcp.keepalive.interval_duration {
            Some(interval_duration) => server_builder.tcp_keepalive_interval(Some(Duration::from_secs(interval_duration))),
            None => server_builder.tcp_keepalive_interval(None),
        };

        server_builder = server_builder
            .http2_only(false)
            .http2_max_header_list_size(environment_configuration.application_server.http.maximum_header_list_size)
            .http2_adaptive_window(environment_configuration.application_server.http.adaptive_window)
            .http2_initial_connection_window_size(Some(environment_configuration.application_server.http.connection_window_size))
            .http2_initial_stream_window_size(Some(environment_configuration.application_server.http.stream_window_size))
            .http2_max_concurrent_streams(u32::MAX)
            .http2_max_frame_size(Some(environment_configuration.application_server.http.maximum_frame_size))
            .http2_max_send_buf_size(environment_configuration.application_server.http.maximum_sending_buffer_size as usize);

        if environment_configuration.application_server.http.enable_connect_protocol {
            server_builder = server_builder.http2_enable_connect_protocol()
        };

        server_builder = match environment_configuration.application_server.http.keepalive {
            Some(ref keepalive) => {
                server_builder
                    .http2_keep_alive_interval(Some(Duration::from_secs(keepalive.interval_duration)))
                    .http2_keep_alive_timeout(Duration::from_secs(keepalive.timeout_duration))
            }
            None => server_builder.http2_keep_alive_interval(None),
        };

        server_builder = match environment_configuration.application_server.http.maximum_pending_accept_reset_streams {
            Some(maximum_pending_accept_reset_streams_) => server_builder.http2_max_pending_accept_reset_streams(Some(maximum_pending_accept_reset_streams_)),
            None => server_builder.http2_max_pending_accept_reset_streams(None),
        };

        let environment_configuration = Arc::new(environment_configuration);

        let mut environment_configuration_ = environment_configuration.clone();

        let service = make_service_fn(
            move |_: &'_ AddrStream| -> _ {
                let router_ = router.clone();

                let environment_configuration__ = environment_configuration_.clone();

                let future = async move {
                    let service_fn = service_fn(
                        move |request: Request| -> _ {
                            let router__ = router_.clone();

                            let environment_configuration___ = environment_configuration__.clone();

                            let future_ = async move {
                                let response = Self::resolve(
                                    router__,
                                    request,
                                    environment_configuration___,
                                )
                                .await;

                                Ok::<_, ErrorVoid>(response)
                            };

                            return future_;
                        },
                    );

                    Ok::<_, ErrorVoid>(service_fn)
                };

                return future;
            },
        );

        let signal_interrupt_future = match Self::create_signal(SignalKind::interrupt()) {
            Ok(signal_interrupt_future_) => signal_interrupt_future_,
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

        let signal_terminate_future = match Self::create_signal(SignalKind::terminate()) {
            Ok(signal_terminate_future_) => signal_terminate_future_,
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

        let graceful_shutdown_signal_future = async {
            select! {
                _ = signal_interrupt_future => {
                    ()
                },
                _ = signal_terminate_future => {
                    ()
                },
            }
        };

        let server_future = async move {
            if let Err(error) = server_builder.serve(service).with_graceful_shutdown(graceful_shutdown_signal_future).await {
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

            return Ok(());
        };

        environment_configuration_ = environment_configuration.clone();

        let update_assets___generate_asset_snapshot_future = async move {
            '_a: loop {
                if let Err(mut error_auditor) = Runner::<(
                    CronJob,
                    Task_<UpdateAssets__GenerateAssetSnapshot>,
                )>::run(
                    environment_configuration_.as_ref().task.update_assets___generate_asset_snapshot.cron_configuration.as_str(),
                    environment_configuration_.clone(),
                    Executor::<Task_<UpdateAssets__GenerateAssetSnapshot>>::execute,
                )
                .await
                {
                    error_auditor.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    );

                    Logger::<Auditor<Error>>::log(&error_auditor);
                }
            }

            return Ok(());
        };

        environment_configuration_ = environment_configuration.clone();

        let generate_aggregated_balance_snapshot_future = async move {
            '_a: loop {
                if let Err(mut error_auditor) = Runner::<(
                    CronJob,
                    Task_<GenerateAggregatedBalanceSnapshot>,
                )>::run(
                    environment_configuration_.as_ref().task.generate_aggregated_balance_snapshot.cron_configuration.as_str(),
                    environment_configuration_.clone(),
                    Executor::<Task_<GenerateAggregatedBalanceSnapshot>>::execute,
                )
                .await
                {
                    error_auditor.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    );

                    Logger::<Auditor<Error>>::log(&error_auditor);
                }
            }

            return Ok(());
        };

        environment_configuration_ = environment_configuration.clone();

        let generate_base_balance_snapshot_future = async move {
            '_a: loop {
                if let Err(mut error_auditor) = Runner::<(
                    CronJob,
                    Task_<GenerateBaseBalanceSnapshot>,
                )>::run(
                    environment_configuration_.as_ref().task.generate_aggregated_balance_snapshot.cron_configuration.as_str(),
                    environment_configuration_.clone(),
                    Executor::<Task_<GenerateBaseBalanceSnapshot>>::execute,
                )
                .await
                {
                    error_auditor.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    );

                    Logger::<Auditor<Error>>::log(&error_auditor);
                }
            }

            return Ok(());
        };

        environment_configuration_ = environment_configuration.clone();

        let update_assets_for_subportfolio_trackable_wallet_future = async move {
            '_a: loop {
                if let Err(mut error_auditor) = Runner::<(
                    CronJob,
                    Task_<UpdateAssetsForSubportfolioTrackableWallet>,
                )>::run(
                    environment_configuration_.as_ref().task.update_assets_for_subportfolio_trackable_wallet.cron_configuration.as_str(),
                    environment_configuration_.clone(),
                    Executor::<Task_<UpdateAssetsForSubportfolioTrackableWallet>>::execute,
                )
                .await
                {
                    error_auditor.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    );

                    Logger::<Auditor<Error>>::log(&error_auditor);
                }
            }

            return Ok(());
        };

        Spawner::<TokioNonBlockingTask>::spawn_into_background(update_assets___generate_asset_snapshot_future);

        Spawner::<TokioNonBlockingTask>::spawn_into_background(generate_aggregated_balance_snapshot_future);

        Spawner::<TokioNonBlockingTask>::spawn_into_background(generate_base_balance_snapshot_future);

        Spawner::<TokioNonBlockingTask>::spawn_into_background(update_assets_for_subportfolio_trackable_wallet_future);

        if let Err(mut error_auditor) = server_future.await {
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

    async fn resolve(router: Arc<Router_>, request: Request, environment_configuration: Arc<EnvironmentConfiguration>) -> Response {
        let (parts, mut body) = request.into_parts();

        let r#match = match router.at(parts.uri.path()) {
            Ok(r#match_) => r#match_,
            Err(_) => {
                return Action::<RouteNotFound>::run(&parts);
            }
        };

        match r#match.value {
            ActionRoute::Task {
                task,
            } => {
                match (
                    task,
                    &parts.method,
                ) {
                    (&Task::ForceExecute, &Method::POST) => {
                        return Action::<Task___ForceExecute>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            environment_configuration,
                        )
                        .await;
                    }
                    (&Task::HealthCheck, &Method::POST) => {
                        return Action::<Task___HealthCheck>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            environment_configuration,
                        )
                        .await;
                    }
                    _ => {}
                }
            }
            ActionRoute::HealthCheck => {
                match &parts.method {
                    &Method::GET => {
                        return Action::<HealthCheck>::run(&parts);
                    }
                    _ => {}
                }
            }
        }

        return Action::<RouteNotFound>::run(&parts);
    }

    fn create_signal(signal_kind: SignalKind) -> Result<impl Future<Output = ()>, Auditor<Error>> {
        let signal = match signal(signal_kind) {
            Ok(mut signal) => {
                async move {
                    signal.recv().await;

                    ()
                }
            }
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

        return Ok(signal);
    }
}

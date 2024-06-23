use super::CommandProcessor;
pub use crate::infrastructure_layer::data::control_type::RunServer;
use crate::{
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            control_type::{
                AssetSnapshot___Create,
                AssetSnapshot___GetHistory,
                AssetSnapshot___GetHistoryForPriceDifferencePercentageCalculating,
                AssetSnapshot___GetHistoryForSubportfolioLink,
                BalanceSnapshot___Create,
                BalanceSnapshot___GetHistory,
                BaseBalanceSnapshot___Create,
                CorsPreflightRequest,
                HealthCheck1,
                HealthCheck2,
                Request,
                Response,
                RouteNotFound,
                SubportfolioAsset___CreateForTrackableWallet,
                SubportfolioAsset___GetAllForSubportfolio,
                SubportfolioAsset___GetAllForSubportfolioLink,
                SubportfolioAsset___Update,
                SubportfolioBaseBalanceSnapshot___GetHistory,
                SubportfolioBaseBalanceSnapshot___GetHistoryForSubportfolioLink,
                SubportfolioLink___Create,
                SubportfolioLink___Delete,
                SubportfolioLink___GetAll,
                SubportfolioLink___Update,
                SubportfolioTrackableWallet___GetAll,
                SubportfolioTrackableWallet___GetAllForSubportfolio,
                SubportfolioTrackableWallet___Update,
                Subportfolio___Create,
                Subportfolio___Delete,
                Subportfolio___GetAll,
                Subportfolio___Update,
            },
            environment_configuration::ENVIRONMENT_CONFIGURATION,
            error::{
                Error,
                Other,
                Runtime,
            },
            void::ErrorVoid,
        },
        functionality::service::creator::Creator,
    },
    presentation_layer::{
        data::action_route::{
            matching::{
                ActionRoute,
                AssetSnapshot,
                BalanceSnapshot,
                BaseBalanceSnapshot,
                Subportfolio,
                SubportfolioAsset,
                SubportfolioBaseBalanceSnapshot,
                SubportfolioLink,
                SubportfolioTrackableWallet,
            },
            ACTION_ROUTE,
        },
        functionality::action::Action,
    },
};
use clickhouse::Client;
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

impl CommandProcessor<RunServer> {
    pub fn process() -> Result<(), Auditor<Error>> {
        let _worker_guard = match Self::configure_logger() {
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

        if let Err(mut error_auditor) = Self::run_runtime() {
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

    fn configure_logger() -> Result<WorkerGuard, Auditor<Error>> {
        let non_blocking;

        let worker_guard;

        let logger_level;

        #[cfg(feature = "file_log")]
        {
            let rolling_file_appender = RollingFileAppender::new(
                Rotation::DAILY,
                ENVIRONMENT_CONFIGURATION.logging.directory_path.0,
                ENVIRONMENT_CONFIGURATION.logging.file_name_prefix.0,
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

    fn run_runtime() -> Result<(), Auditor<Error>> {
        if ENVIRONMENT_CONFIGURATION.tokio_runtime.maximum_blocking_threads_quantity == 0
            || ENVIRONMENT_CONFIGURATION.tokio_runtime.worker_threads_quantity == 0
            || ENVIRONMENT_CONFIGURATION.tokio_runtime.worker_thread_stack_size < (1024 * 1024)
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
            .max_blocking_threads(ENVIRONMENT_CONFIGURATION.tokio_runtime.maximum_blocking_threads_quantity)
            .worker_threads(ENVIRONMENT_CONFIGURATION.tokio_runtime.worker_threads_quantity)
            .thread_stack_size(ENVIRONMENT_CONFIGURATION.tokio_runtime.worker_thread_stack_size)
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

        if let Err(mut error_auditor) = runtime.block_on(Self::run_server()) {
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
            ACTION_ROUTE.balance_snapshot.get_history,
            ActionRoute::BalanceSnapshot {
                balance_snapshot: BalanceSnapshot::GetHistory,
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
            ACTION_ROUTE.balance_snapshot.create,
            ActionRoute::BalanceSnapshot {
                balance_snapshot: BalanceSnapshot::Create,
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
            ACTION_ROUTE.base_balance_snapshot.create,
            ActionRoute::BaseBalanceSnapshot {
                base_balance_snapshot: BaseBalanceSnapshot::Create,
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
            ACTION_ROUTE.subportfolio_base_balance_snapshot.get_history,
            ActionRoute::SubportfolioBaseBalanceSnapshot {
                subportfolio_base_balance_snapshot: SubportfolioBaseBalanceSnapshot::GetHistory,
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
            ACTION_ROUTE.subportfolio_base_balance_snapshot.get_history_for_subportfolio_link,
            ActionRoute::SubportfolioBaseBalanceSnapshot {
                subportfolio_base_balance_snapshot: SubportfolioBaseBalanceSnapshot::GetHistoryForSubportfolioLink,
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
            ACTION_ROUTE.asset_snapshot.create,
            ActionRoute::AssetSnapshot {
                asset_snapshot: AssetSnapshot::Create,
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
            ACTION_ROUTE.asset_snapshot.get_history,
            ActionRoute::AssetSnapshot {
                asset_snapshot: AssetSnapshot::GetHistory,
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
            ACTION_ROUTE.asset_snapshot.get_history_for_subportfolio_link,
            ActionRoute::AssetSnapshot {
                asset_snapshot: AssetSnapshot::GetHistoryForSubportfolioLink,
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
            ACTION_ROUTE.asset_snapshot.get_history_for_price_difference_percentage_calculating,
            ActionRoute::AssetSnapshot {
                asset_snapshot: AssetSnapshot::GetHistoryForPriceDifferencePercentageCalculating,
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
            ACTION_ROUTE.subportfolio.create,
            ActionRoute::Subportfolio {
                subportfolio: Subportfolio::Create,
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
            ACTION_ROUTE.subportfolio.delete,
            ActionRoute::Subportfolio {
                subportfolio: Subportfolio::Delete,
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
            ACTION_ROUTE.subportfolio.get_all,
            ActionRoute::Subportfolio {
                subportfolio: Subportfolio::GetAll,
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
            ACTION_ROUTE.subportfolio.update,
            ActionRoute::Subportfolio {
                subportfolio: Subportfolio::Update,
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
            ACTION_ROUTE.subportfolio_asset.update,
            ActionRoute::SubportfolioAsset {
                subportfolio_asset: SubportfolioAsset::Update,
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
            ACTION_ROUTE.subportfolio_asset.get_all_for_subportfolio,
            ActionRoute::SubportfolioAsset {
                subportfolio_asset: SubportfolioAsset::GetAllForSubportfolio,
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
            ACTION_ROUTE.subportfolio_asset.get_all_for_subportfolio_link,
            ActionRoute::SubportfolioAsset {
                subportfolio_asset: SubportfolioAsset::GetAllForSubportfolioLink,
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
            ACTION_ROUTE.subportfolio_asset.create_for_trackable_wallet,
            ActionRoute::SubportfolioAsset {
                subportfolio_asset: SubportfolioAsset::CreateForTrackableWallet,
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
            ACTION_ROUTE.subportfolio_link.create,
            ActionRoute::SubportfolioLink {
                subportfolio_link: SubportfolioLink::Create,
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
            ACTION_ROUTE.subportfolio_link.delete,
            ActionRoute::SubportfolioLink {
                subportfolio_link: SubportfolioLink::Delete,
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
            ACTION_ROUTE.subportfolio_link.get_all,
            ActionRoute::SubportfolioLink {
                subportfolio_link: SubportfolioLink::GetAll,
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
            ACTION_ROUTE.subportfolio_link.update,
            ActionRoute::SubportfolioLink {
                subportfolio_link: SubportfolioLink::Update,
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
            ACTION_ROUTE.subportfolio_trackable_wallet.get_all,
            ActionRoute::SubportfolioTrackableWallet {
                subportfolio_trackable_wallet: SubportfolioTrackableWallet::GetAll,
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
            ACTION_ROUTE.subportfolio_trackable_wallet.get_all_for_subportfolio,
            ActionRoute::SubportfolioTrackableWallet {
                subportfolio_trackable_wallet: SubportfolioTrackableWallet::GetAllForSubportfolio,
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
            ACTION_ROUTE.subportfolio_trackable_wallet.update,
            ActionRoute::SubportfolioTrackableWallet {
                subportfolio_trackable_wallet: SubportfolioTrackableWallet::Update,
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
            ACTION_ROUTE.health_check_1,
            ActionRoute::HealthCheck1,
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
            ACTION_ROUTE.health_check_2,
            ActionRoute::HealthCheck2,
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

    async fn run_server() -> Result<(), Auditor<Error>> {
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

        // TODO: Right now, the 'clickhouse' crate is not able to create a connection pool and make tcp connection.
        let clickhouse_client = match Creator::<Client>::create(
            ENVIRONMENT_CONFIGURATION.resource.clickhouse.url.0,
            ENVIRONMENT_CONFIGURATION.resource.clickhouse.user.0,
            ENVIRONMENT_CONFIGURATION.resource.clickhouse.password.0,
            ENVIRONMENT_CONFIGURATION.resource.clickhouse.database.0,
        ) {
            Ok(clickhouse_client_) => clickhouse_client_,
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

        let mut application_http_socket_address_registry = match ENVIRONMENT_CONFIGURATION.application_server.tcp.socket_address.0.to_socket_addrs() {
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
            .tcp_keepalive_retries(ENVIRONMENT_CONFIGURATION.application_server.tcp.keepalive.retries_quantity)
            .tcp_nodelay(ENVIRONMENT_CONFIGURATION.application_server.tcp.nodelay)
            .tcp_sleep_on_accept_errors(ENVIRONMENT_CONFIGURATION.application_server.tcp.sleep_on_accept_errors);

        server_builder = match ENVIRONMENT_CONFIGURATION.application_server.tcp.keepalive.duration {
            Some(duration) => server_builder.tcp_keepalive(Some(Duration::from_secs(duration))),
            None => server_builder.tcp_keepalive(None),
        };

        server_builder = match ENVIRONMENT_CONFIGURATION.application_server.tcp.keepalive.interval_duration {
            Some(interval_duration) => server_builder.tcp_keepalive_interval(Some(Duration::from_secs(interval_duration))),
            None => server_builder.tcp_keepalive_interval(None),
        };

        server_builder = server_builder
            .http2_only(false)
            .http2_max_header_list_size(ENVIRONMENT_CONFIGURATION.application_server.http.maximum_header_list_size)
            .http2_adaptive_window(ENVIRONMENT_CONFIGURATION.application_server.http.adaptive_window)
            .http2_initial_connection_window_size(Some(ENVIRONMENT_CONFIGURATION.application_server.http.connection_window_size))
            .http2_initial_stream_window_size(Some(ENVIRONMENT_CONFIGURATION.application_server.http.stream_window_size))
            .http2_max_concurrent_streams(u32::MAX)
            .http2_max_frame_size(Some(ENVIRONMENT_CONFIGURATION.application_server.http.maximum_frame_size))
            .http2_max_send_buf_size(ENVIRONMENT_CONFIGURATION.application_server.http.maximum_sending_buffer_size as usize);

        if ENVIRONMENT_CONFIGURATION.application_server.http.enable_connect_protocol {
            server_builder = server_builder.http2_enable_connect_protocol()
        };

        server_builder = match ENVIRONMENT_CONFIGURATION.application_server.http.keepalive {
            Some(ref keepalive) => {
                server_builder
                    .http2_keep_alive_interval(Some(Duration::from_secs(keepalive.interval_duration)))
                    .http2_keep_alive_timeout(Duration::from_secs(keepalive.timeout_duration))
            }
            None => server_builder.http2_keep_alive_interval(None),
        };

        server_builder = match ENVIRONMENT_CONFIGURATION.application_server.http.maximum_pending_accept_reset_streams {
            Some(maximum_pending_accept_reset_streams_) => server_builder.http2_max_pending_accept_reset_streams(Some(maximum_pending_accept_reset_streams_)),
            None => server_builder.http2_max_pending_accept_reset_streams(None),
        };

        let service = make_service_fn(
            move |_: &'_ AddrStream| -> _ {
                let router_ = router.clone();

                let clickhouse_client_ = clickhouse_client.clone();

                let future = async move {
                    let service_fn = service_fn(
                        move |request: Request| -> _ {
                            let router__ = router_.clone();

                            let clickhouse_client__ = clickhouse_client_.clone();

                            let future_ = async move {
                                let response = Self::resolve(
                                    router__,
                                    request,
                                    clickhouse_client__,
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
    }

    async fn resolve(router: Arc<Router_>, request: Request, clickhouse_client: Client) -> Response {
        let (parts, mut body) = request.into_parts();

        let r#match = match router.at(parts.uri.path()) {
            Ok(r#match_) => r#match_,
            Err(_) => {
                return Action::<RouteNotFound>::run(&parts);
            }
        };

        if parts.method == Method::OPTIONS {
            return Action::<CorsPreflightRequest>::run(&parts);
        }

        match r#match.value {
            ActionRoute::BalanceSnapshot {
                balance_snapshot,
            } => {
                match (
                    balance_snapshot,
                    &parts.method,
                ) {
                    (
                        &BalanceSnapshot::GetHistory,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<BalanceSnapshot___GetHistory>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    (&BalanceSnapshot::Create, &Method::POST) => {
                        return Action::<BalanceSnapshot___Create>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    _ => {}
                }
            }
            ActionRoute::BaseBalanceSnapshot {
                base_balance_snapshot,
            } => {
                match (
                    base_balance_snapshot,
                    &parts.method,
                ) {
                    (&BaseBalanceSnapshot::Create, &Method::POST) => {
                        return Action::<BaseBalanceSnapshot___Create>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    _ => {}
                }
            }
            ActionRoute::SubportfolioBaseBalanceSnapshot {
                subportfolio_base_balance_snapshot,
            } => {
                match (
                    subportfolio_base_balance_snapshot,
                    &parts.method,
                ) {
                    (
                        &SubportfolioBaseBalanceSnapshot::GetHistory,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<SubportfolioBaseBalanceSnapshot___GetHistory>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    (
                        &SubportfolioBaseBalanceSnapshot::GetHistoryForSubportfolioLink,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<SubportfolioBaseBalanceSnapshot___GetHistoryForSubportfolioLink>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    _ => {}
                }
            }
            ActionRoute::AssetSnapshot {
                asset_snapshot,
            } => {
                match (
                    asset_snapshot,
                    &parts.method,
                ) {
                    (&AssetSnapshot::Create, &Method::POST) => {
                        return Action::<AssetSnapshot___Create>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    (
                        &AssetSnapshot::GetHistory,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<AssetSnapshot___GetHistory>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    (
                        &AssetSnapshot::GetHistoryForSubportfolioLink,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<AssetSnapshot___GetHistoryForSubportfolioLink>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    (
                        &AssetSnapshot::GetHistoryForPriceDifferencePercentageCalculating,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<AssetSnapshot___GetHistoryForPriceDifferencePercentageCalculating>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    _ => {}
                }
            }
            ActionRoute::Subportfolio {
                subportfolio,
            } => {
                match (
                    subportfolio,
                    &parts.method,
                ) {
                    (&Subportfolio::Create, &Method::POST) => {
                        return Action::<Subportfolio___Create>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    (&Subportfolio::Delete, &Method::POST) => {
                        return Action::<Subportfolio___Delete>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    (
                        &Subportfolio::GetAll,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<Subportfolio___GetAll>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    (&Subportfolio::Update, &Method::POST) => {
                        return Action::<Subportfolio___Update>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    _ => {}
                }
            }
            ActionRoute::SubportfolioAsset {
                subportfolio_asset,
            } => {
                match (
                    subportfolio_asset,
                    &parts.method,
                ) {
                    (&SubportfolioAsset::Update, &Method::POST) => {
                        return Action::<SubportfolioAsset___Update>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    (
                        &SubportfolioAsset::GetAllForSubportfolio,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<SubportfolioAsset___GetAllForSubportfolio>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    (
                        &SubportfolioAsset::GetAllForSubportfolioLink,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<SubportfolioAsset___GetAllForSubportfolioLink>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    (
                        &SubportfolioAsset::CreateForTrackableWallet,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<SubportfolioAsset___CreateForTrackableWallet>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    _ => {}
                }
            }
            ActionRoute::SubportfolioLink {
                subportfolio_link,
            } => {
                match (
                    subportfolio_link,
                    &parts.method,
                ) {
                    (
                        &SubportfolioLink::Create,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<SubportfolioLink___Create>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    (
                        &SubportfolioLink::Delete,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<SubportfolioLink___Delete>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    (
                        &SubportfolioLink::GetAll,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<SubportfolioLink___GetAll>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    (
                        &SubportfolioLink::Update,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<SubportfolioLink___Update>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    _ => {}
                }
            }
            ActionRoute::SubportfolioTrackableWallet {
                subportfolio_trackable_wallet,
            } => {
                match (
                    subportfolio_trackable_wallet,
                    &parts.method,
                ) {
                    (
                        &SubportfolioTrackableWallet::GetAll,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<SubportfolioTrackableWallet___GetAll>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    (
                        &SubportfolioTrackableWallet::GetAllForSubportfolio,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<SubportfolioTrackableWallet___GetAllForSubportfolio>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    (
                        &SubportfolioTrackableWallet::Update,
                        // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                        &Method::POST,
                    ) => {
                        return Action::<SubportfolioTrackableWallet___Update>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
                    }
                    _ => {}
                }
            }
            ActionRoute::HealthCheck1 => {
                match &parts.method {
                    &Method::GET => {
                        return Action::<HealthCheck1>::run(&parts);
                    }
                    _ => {}
                }
            }
            ActionRoute::HealthCheck2 => {
                match &parts.method {
                    // Should be GET. But due to restrictions of third-party services, the method is put in Post.
                    &Method::POST => {
                        return Action::<HealthCheck2>::run(
                            &mut body,
                            &parts,
                            &r#match.params,
                            clickhouse_client,
                        )
                        .await;
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

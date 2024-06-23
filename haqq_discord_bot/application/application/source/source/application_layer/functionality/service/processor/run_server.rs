use super::Processor;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::control_type::HealthCheck;
use crate::infrastructure_layer::data::control_type::Request;
use crate::infrastructure_layer::data::control_type::Response;
use crate::infrastructure_layer::data::control_type::RouteNotFound;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::data::void::ErrorVoid;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use crate::presentation_layer::data::action_route::matching::ActionRoute;
use crate::presentation_layer::data::action_route::ACTION_ROUTE;
use crate::presentation_layer::functionality::action::Action;
use hyper::server::conn::AddrStream;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
use hyper::Method;
use hyper::Server;
use matchit::Router;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub use crate::infrastructure_layer::data::control_type::RunServer;

impl Processor<RunServer> {
    pub async fn process(environment_configuration: Arc<EnvironmentConfiguration>) -> Result<(), Auditor<Error>> {
        '_a: loop {
            if let Err(mut error_auditor) = Self::process_1(environment_configuration.clone()).await {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                Logger::<Auditor<Error>>::log(&error_auditor);

                sleep(Duration::from_secs(5)).await;
            }
        }

        return Ok(());
    }

    async fn process_1(environment_configuration: Arc<EnvironmentConfiguration>) -> Result<(), Auditor<Error>> {
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

        let environment_configuration_ = environment_configuration.as_ref();

        let mut application_http_socket_address_registry = match environment_configuration_.application_server.tcp.socket_address.to_socket_addrs() {
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
            .tcp_keepalive_retries(environment_configuration_.application_server.tcp.keepalive.retries_quantity)
            .tcp_nodelay(environment_configuration_.application_server.tcp.nodelay)
            .tcp_sleep_on_accept_errors(environment_configuration_.application_server.tcp.sleep_on_accept_errors);

        server_builder = match environment_configuration_.application_server.tcp.keepalive.duration {
            Some(duration) => server_builder.tcp_keepalive(Some(Duration::from_secs(duration))),
            None => server_builder.tcp_keepalive(None),
        };

        server_builder = match environment_configuration_.application_server.tcp.keepalive.interval_duration {
            Some(interval_duration) => server_builder.tcp_keepalive_interval(Some(Duration::from_secs(interval_duration))),
            None => server_builder.tcp_keepalive_interval(None),
        };

        server_builder = server_builder
            .http2_only(false)
            .http2_max_header_list_size(environment_configuration_.application_server.http.maximum_header_list_size)
            .http2_adaptive_window(environment_configuration_.application_server.http.adaptive_window)
            .http2_initial_connection_window_size(Some(environment_configuration_.application_server.http.connection_window_size))
            .http2_initial_stream_window_size(Some(environment_configuration_.application_server.http.stream_window_size))
            .http2_max_concurrent_streams(u32::MAX)
            .http2_max_frame_size(Some(environment_configuration_.application_server.http.maximum_frame_size))
            .http2_max_send_buf_size(environment_configuration_.application_server.http.maximum_sending_buffer_size as usize);

        if environment_configuration_.application_server.http.enable_connect_protocol {
            server_builder = server_builder.http2_enable_connect_protocol()
        };

        server_builder = match environment_configuration_.application_server.http.keepalive {
            Some(ref keepalive) => server_builder
                .http2_keep_alive_interval(Some(Duration::from_secs(keepalive.interval_duration)))
                .http2_keep_alive_timeout(Duration::from_secs(keepalive.timeout_duration)),
            None => server_builder.http2_keep_alive_interval(None),
        };

        server_builder = match environment_configuration_.application_server.http.maximum_pending_accept_reset_streams {
            Some(maximum_pending_accept_reset_streams_) => server_builder.http2_max_pending_accept_reset_streams(Some(maximum_pending_accept_reset_streams_)),
            None => server_builder.http2_max_pending_accept_reset_streams(None),
        };

        let environment_configuration__ = environment_configuration.clone();

        let service = make_service_fn(
            move |_: &'_ AddrStream| -> _ {
                let router_ = router.clone();

                let environment_configuration___ = environment_configuration__.clone();

                let future = async move {
                    let service_fn = service_fn(
                        move |request: Request| -> _ {
                            let router__ = router_.clone();

                            let environment_configuration____ = environment_configuration___.clone();

                            let future_ = async move {
                                let response = Self::resolve(
                                    router__,
                                    request,
                                    environment_configuration____,
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

        if let Err(error) = server_builder.serve(service).await {
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
        };

        return Ok(());
    }

    fn create_router() -> Result<Router<ActionRoute>, Auditor<Error>> {
        let mut router = Router::new();

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

    async fn resolve(
        router: Arc<Router<ActionRoute>>,
        request: Request,
        _environment_configuration: Arc<EnvironmentConfiguration>,
    ) -> Response {
        let (parts, mut _body) = request.into_parts();

        let r#match = match router.at(parts.uri.path()) {
            Ok(r#match_) => r#match_,
            Err(_) => {
                return Action::<RouteNotFound>::run(&parts);
            }
        };

        match r#match.value {
            ActionRoute::HealthCheck => {
                match &parts.method {
                    &Method::GET => {
                        return Action::<HealthCheck>::run(&parts);
                    }
                    _ => {}
                }
            },
        }

        return Action::<RouteNotFound>::run(&parts);
    }
}

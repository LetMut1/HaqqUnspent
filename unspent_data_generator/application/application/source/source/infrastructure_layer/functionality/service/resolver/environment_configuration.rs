use super::Resolver;
use crate::infrastructure_layer::data::{
    auditor::Auditor,
    backtrace::BacktracePart,
    environment_configuration::{
        environment_configuration_file::EnvironmentConfigurationFile,
        ApplicationServer,
        Coingecko,
        DataProxy,
        Environment,
        EnvironmentConfiguration,
        GenerateAggregatedBalanceSnapshot,
        GenerateBaseBalanceSnapshot,
        Http,
        HttpKeepalive,
        Logging,
        Postgresql,
        Pro,
        RemoteService,
        Resource,
        Security,
        Task,
        Tcp,
        TcpKeepalive,
        TokioRuntime,
        UpdateAssetsForSubportfolioTrackableWallet,
        UpdateAssets__GenerateAssetSnapshot,
    },
    error::{
        Error,
        Other,
        Runtime,
    },
};
use std::{
    fs::read_to_string,
    path::Path,
};
use toml::from_str;

impl Resolver<EnvironmentConfiguration> {
    const PRODUCTION_ENVIRONMENT_DIRECTORY_NAME: &'static str = "production";
    const DEVELOPMENT_ENVIRONMENT_DIRECTORY_NAME: &'static str = "development";
    const LOCAL_DEVELOPMENT_ENVIRONMENT_DIRECTORY_NAME: &'static str = "local_development";
    const ENVIRONMENT_FILE_NAME: &'static str = "environment.toml";

    pub fn load_from_file<'a>(environment_configuration_directory_path: &'a str) -> Result<EnvironmentConfiguration, Auditor<Error>> {
        let production_environment_file_path = format!(
            "{}/{}/{}",
            environment_configuration_directory_path,
            Self::PRODUCTION_ENVIRONMENT_DIRECTORY_NAME,
            Self::ENVIRONMENT_FILE_NAME,
        );

        let production_environment_file_path_ = Path::new(production_environment_file_path.as_str());

        let production_environment_file_path_is_exist = match production_environment_file_path_.try_exists() {
            Ok(production_environment_file_path_is_exist_) => production_environment_file_path_is_exist_,
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

        let (environment, environment_file_data) = if production_environment_file_path_is_exist {
            let environment_file_data_ = match read_to_string(production_environment_file_path_) {
                Ok(environment_file_data__) => environment_file_data__,
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

            (
                Environment::Production,
                environment_file_data_,
            )
        } else {
            let local_development_environment_file_path = format!(
                "{}/{}/{}",
                environment_configuration_directory_path,
                Self::LOCAL_DEVELOPMENT_ENVIRONMENT_DIRECTORY_NAME,
                Self::ENVIRONMENT_FILE_NAME,
            );

            let local_development_environment_file_path_ = Path::new(local_development_environment_file_path.as_str());

            let local_development_environment_file_path_is_exist = match local_development_environment_file_path_.try_exists() {
                Ok(local_development_environment_file_path_is_exist_) => local_development_environment_file_path_is_exist_,
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

            if local_development_environment_file_path_is_exist {
                let environment_file_data_ = match read_to_string(local_development_environment_file_path_) {
                    Ok(environment_file_data__) => environment_file_data__,
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

                (
                    Environment::LocalDevelopment,
                    environment_file_data_,
                )
            } else {
                let development_environment_file_path = format!(
                    "{}/{}/{}",
                    environment_configuration_directory_path,
                    Self::DEVELOPMENT_ENVIRONMENT_DIRECTORY_NAME,
                    Self::ENVIRONMENT_FILE_NAME,
                );

                let development_environment_file_path_ = Path::new(development_environment_file_path.as_str());

                let development_environment_file_path_is_exist = match development_environment_file_path_.try_exists() {
                    Ok(development_environment_file_path_is_exist_) => development_environment_file_path_is_exist_,
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

                if development_environment_file_path_is_exist {
                    let environment_file_data_ = match read_to_string(development_environment_file_path_) {
                        Ok(environment_file_data__) => environment_file_data__,
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

                    (
                        Environment::Development,
                        environment_file_data_,
                    )
                } else {
                    return Err(
                        Auditor::<Error>::new(
                            Error::Logic {
                                message: "The environment.toml file does not exist.",
                            },
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
            }
        };

        let environment_configuration_file = match from_str::<EnvironmentConfigurationFile>(environment_file_data.as_str()) {
            Ok(environment_configuration_file_) => environment_configuration_file_,
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

        let environment_configuration = {
            let application_server = {
                let tcp = {
                    let keepalive = {
                        let duration = if environment_configuration_file.application_server.tcp.keepalive.duration.is_exist {
                            Some(environment_configuration_file.application_server.tcp.keepalive.duration.value)
                        } else {
                            None
                        };

                        let interval_duration = if environment_configuration_file.application_server.tcp.keepalive.interval_duration.is_exist {
                            Some(environment_configuration_file.application_server.tcp.keepalive.interval_duration.value)
                        } else {
                            None
                        };

                        let retries_quantity = if environment_configuration_file.application_server.tcp.keepalive.retries_quantity.is_exist {
                            Some(environment_configuration_file.application_server.tcp.keepalive.retries_quantity.value)
                        } else {
                            None
                        };

                        TcpKeepalive {
                            duration,
                            interval_duration,
                            retries_quantity,
                        }
                    };

                    Tcp {
                        socket_address: environment_configuration_file.application_server.tcp.socket_address.value,
                        nodelay: environment_configuration_file.application_server.tcp.nodelay.value,
                        sleep_on_accept_errors: environment_configuration_file.application_server.tcp.sleep_on_accept_errors.value,
                        keepalive,
                    }
                };

                let http = {
                    let keepalive = if environment_configuration_file.application_server.http.keepalive.is_exist {
                        Some(
                            HttpKeepalive {
                                interval_duration: environment_configuration_file.application_server.http.keepalive.interval_duration.value,
                                timeout_duration: environment_configuration_file.application_server.http.keepalive.timeout_duration.value,
                            },
                        )
                    } else {
                        None
                    };

                    let maximum_pending_accept_reset_streams = if environment_configuration_file.application_server.http.maximum_pending_accept_reset_streams.is_exist {
                        Some(environment_configuration_file.application_server.http.maximum_pending_accept_reset_streams.value)
                    } else {
                        None
                    };

                    Http {
                        adaptive_window: environment_configuration_file.application_server.http.adaptive_window.value,
                        connection_window_size: environment_configuration_file.application_server.http.connection_window_size.value,
                        stream_window_size: environment_configuration_file.application_server.http.stream_window_size.value,
                        maximum_frame_size: environment_configuration_file.application_server.http.maximum_frame_size.value,
                        maximum_sending_buffer_size: environment_configuration_file.application_server.http.maximum_sending_buffer_size.value,
                        enable_connect_protocol: environment_configuration_file.application_server.http.enable_connect_protocol.value,
                        maximum_header_list_size: environment_configuration_file.application_server.http.maximum_header_list_size.value,
                        maximum_pending_accept_reset_streams,
                        keepalive,
                    }
                };

                ApplicationServer {
                    tcp,
                    http,
                }
            };

            EnvironmentConfiguration {
                environment,
                tokio_runtime: TokioRuntime {
                    maximum_blocking_threads_quantity: environment_configuration_file.tokio_runtime.maximum_blocking_threads_quantity.value,
                    worker_threads_quantity: environment_configuration_file.tokio_runtime.worker_threads_quantity.value,
                    worker_thread_stack_size: environment_configuration_file.tokio_runtime.worker_thread_stack_size.value,
                },
                application_server,
                remote_service: RemoteService {
                    data_proxy: DataProxy {
                        url: environment_configuration_file.remote_service.data_proxy.url.value,
                        server_access_token: environment_configuration_file.remote_service.data_proxy.server_access_token.value,
                    },
                    coingecko: Coingecko {
                        pro: Pro {
                            url: environment_configuration_file.remote_service.coingecko.pro.url.value,
                            api_key: environment_configuration_file.remote_service.coingecko.pro.api_key.value,
                        },
                    },
                },
                logging: Logging {
                    directory_path: environment_configuration_file.logging.directory_path.value,
                    file_name_prefix: environment_configuration_file.logging.file_name_prefix.value,
                },
                security: Security {
                    server_access_token: environment_configuration_file.security.server_access_token.value,
                },
                resource: Resource {
                    postgresql: Postgresql {
                        configuration: environment_configuration_file.resource.postgresql.configuration.value,
                    },
                },
                task: Task {
                    generate_aggregated_balance_snapshot: GenerateAggregatedBalanceSnapshot {
                        cron_configuration: environment_configuration_file.task.generate_aggregated_balance_snapshot.cron_configuration.value,
                    },
                    generate_base_balance_snapshot: GenerateBaseBalanceSnapshot {
                        cron_configuration: environment_configuration_file.task.generate_base_balance_snapshot.cron_configuration.value,
                    },
                    update_assets___generate_asset_snapshot: UpdateAssets__GenerateAssetSnapshot {
                        cron_configuration: environment_configuration_file.task.update_assets___generate_asset_snapshot.cron_configuration.value,
                    },
                    update_assets_for_subportfolio_trackable_wallet: UpdateAssetsForSubportfolioTrackableWallet {
                        cron_configuration: environment_configuration_file.task.update_assets_for_subportfolio_trackable_wallet.cron_configuration.value,
                    },
                },
            }
        };

        return Ok(environment_configuration);
    }
}

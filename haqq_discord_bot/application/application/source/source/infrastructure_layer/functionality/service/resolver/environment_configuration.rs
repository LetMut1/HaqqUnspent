use super::Resolver;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::environment_configuration::environment_configuration_file::EnvironmentConfigurationFile;
use crate::infrastructure_layer::data::environment_configuration::Application;
use crate::infrastructure_layer::data::environment_configuration::ApplicationServer;
use crate::infrastructure_layer::data::environment_configuration::Bot;
use crate::infrastructure_layer::data::environment_configuration::Discord;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::environment_configuration::Guild;
use crate::infrastructure_layer::data::environment_configuration::Http;
use crate::infrastructure_layer::data::environment_configuration::HttpKeepalive;
use crate::infrastructure_layer::data::environment_configuration::Logging;
use crate::infrastructure_layer::data::environment_configuration::NoncontextParameters;
use crate::infrastructure_layer::data::environment_configuration::Postgresql;
use crate::infrastructure_layer::data::environment_configuration::RemoteService;
use crate::infrastructure_layer::data::environment_configuration::Resource;
use crate::infrastructure_layer::data::environment_configuration::Security;
use crate::infrastructure_layer::data::environment_configuration::Tcp;
use crate::infrastructure_layer::data::environment_configuration::TcpKeepalive;
use crate::infrastructure_layer::data::environment_configuration::TokioRuntime;
use crate::infrastructure_layer::data::environment_configuration::EvmNode;
use crate::infrastructure_layer::data::environment_configuration::Haqq;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use std::fs::read_to_string;
use std::path::Path;
use toml::from_str;

impl Resolver<EnvironmentConfiguration> {
    const PRODUCTION_ENVIRONMENT_DIRECTORY_NAME: &'static str = "production";
    const DEVELOPMENT_ENVIRONMENT_DIRECTORY_NAME: &'static str = "development";
    const LOCAL_DEVELOPMENT_ENVIRONMENT_DIRECTORY_NAME: &'static str = "local_development";
    const ENVIRONMENT_FILE_NAME: &'static str = "environment.toml";

    pub fn load_from_file<'a>(environment_configuration_directory_path: &'a str) -> Result<EnvironmentConfiguration, Auditor<Error>> {
        let environment_file_path = format!(
            "{}/{}",
            environment_configuration_directory_path,
            Self::ENVIRONMENT_FILE_NAME,
        );

        let environment_file_path_ = Path::new(environment_file_path.as_str());

        let environment_file_path_is_exist = match environment_file_path_.try_exists() {
            Ok(environment_file_path_is_exist_) => environment_file_path_is_exist_,
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

        let environment_file_data = if environment_file_path_is_exist {
            let environment_file_data_ = match read_to_string(environment_file_path_) {
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

            environment_file_data_
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
                remote_service: RemoteService {
                    discord: Discord {
                        guild: Guild {
                            id: environment_configuration_file.remote_service.discord.guild.id.value,
                        },
                        application: Application {
                            bot: Bot {
                                id: environment_configuration_file.remote_service.discord.application.bot.id.value,
                                role_id: environment_configuration_file.remote_service.discord.application.bot.role_id.value,
                                public_key: environment_configuration_file.remote_service.discord.application.bot.public_key.value,
                                token: environment_configuration_file.remote_service.discord.application.bot.token.value,
                            },
                        },
                    },
                    haqq: Haqq {
                        evm_node: EvmNode {
                            url: environment_configuration_file.remote_service.haqq.evm_node.url.value,
                        },
                    },
                },
                noncontext_parameters: NoncontextParameters {
                    raffle_stake_updating_cron_configuration: environment_configuration_file.noncontext_parameters.raffle_stake_updating_cron_configuration.value,
                    aislm_stake_streshold_quantity_for_stakers_club_role: environment_configuration_file
                        .noncontext_parameters
                        .aislm_stake_streshold_quantity_for_stakers_club_role
                        .value,
                    discord_roles_updating_cron_configuration: environment_configuration_file.noncontext_parameters.discord_roles_updating_cron_configuration.value,
                    algorithm_repetition_in_error_case_quantity: environment_configuration_file.noncontext_parameters.algorithm_repetition_in_error_case_quantity.value,
                    wallet_verification_process_duraction_minutes: environment_configuration_file.noncontext_parameters.wallet_verification_process_duraction_minutes.value,
                },
                tokio_runtime: TokioRuntime {
                    maximum_blocking_threads_quantity: environment_configuration_file.tokio_runtime.maximum_blocking_threads_quantity.value,
                    worker_threads_quantity: environment_configuration_file.tokio_runtime.worker_threads_quantity.value,
                    worker_thread_stack_size: environment_configuration_file.tokio_runtime.worker_thread_stack_size.value,
                },
                application_server,
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
            }
        };

        return Ok(environment_configuration);
    }
}

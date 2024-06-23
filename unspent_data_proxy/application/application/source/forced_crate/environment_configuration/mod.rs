#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_match,
    clippy::explicit_into_iter_loop,
    clippy::module_inception,
    clippy::needless_continue,
    clippy::needless_lifetimes,
    clippy::needless_return,
    clippy::new_without_default,
    clippy::redundant_pattern_matching,
    clippy::single_match_else,
    clippy::string_add,
    clippy::too_many_arguments,
    clippy::trait_duplication_in_bounds,
    clippy::unused_unit,
    clippy::empty_enum,
    clippy::let_unit_value,
    clippy::let_and_return,
    clippy::manual_range_contains,
    clippy::enum_variant_names,
    clippy::type_complexity,
    clippy::explicit_auto_deref,
    clippy::redundant_static_lifetimes,
    clippy::manual_map
)]
#![deny(
    clippy::unnecessary_cast,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::fallible_impl_from,
    clippy::float_cmp_const,
    clippy::from_iter_instead_of_collect,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wild_err_arm,
    clippy::mem_forget,
    clippy::missing_enforced_import_renames,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_for_each,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::rc_mutex,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::string_add_assign,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values
)]

pub mod environment_configuration {
    use self::sealed::Sealed;

    pub enum Environment {
        Production,
        Development,
        LocalDevelopment,
    }

    pub struct EnvironmentConfiguration<T>
    where
        T: Sealed,
    {
        pub environment: Environment,
        pub tokio_runtime: TokioRuntime,
        pub application_server: ApplicationServer<T>,
        pub remote_service: RemoteService<T>,
        pub logging: Logging<T>,
        pub security: Security<T>,
        pub resource: Resource<T>,
    }

    pub struct TokioRuntime {
        pub maximum_blocking_threads_quantity: usize,
        pub worker_threads_quantity: usize,
        pub worker_thread_stack_size: usize,
    }

    pub struct ApplicationServer<T>
    where
        T: Sealed,
    {
        pub tcp: Tcp<T>,
        pub http: Http,
    }

    pub struct Tcp<T>
    where
        T: Sealed,
    {
        pub socket_address: T,
        pub nodelay: bool,
        pub sleep_on_accept_errors: bool,
        pub keepalive: TcpKeepalive,
    }

    pub struct TcpKeepalive {
        pub duration: Option<u64>,
        pub interval_duration: Option<u64>,
        pub retries_quantity: Option<u32>,
    }

    pub struct Http {
        pub adaptive_window: bool,
        pub connection_window_size: u32,
        pub stream_window_size: u32,
        pub maximum_frame_size: u32,
        pub maximum_sending_buffer_size: u32,
        pub enable_connect_protocol: bool,
        pub maximum_header_list_size: u32,
        pub maximum_pending_accept_reset_streams: Option<usize>,
        pub keepalive: Option<HttpKeepalive>,
    }

    pub struct HttpKeepalive {
        pub interval_duration: u64,
        pub timeout_duration: u64,
    }

    pub struct RemoteService<T>
    where
        T: Sealed,
    {
        pub user_authorization: UserAuthorization<T>,
    }

    pub struct UserAuthorization<T>
    where
        T: Sealed,
    {
        pub url: T,
    }

    pub struct Logging<T>
    where
        T: Sealed,
    {
        pub directory_path: T,
        pub file_name_prefix: T,
    }

    pub struct Security<T>
    where
        T: Sealed,
    {
        pub server_access_token: T,
    }

    pub struct Resource<T>
    where
        T: Sealed,
    {
        pub clickhouse: Clickhouse<T>,
        pub postgresql: Postgresql<T>,
    }

    pub struct Clickhouse<T>
    where
        T: Sealed,
    {
        pub url: T,
        pub user: T,
        pub password: T,
        pub database: T,
    }

    pub struct Postgresql<T>
    where
        T: Sealed,
    {
        pub selecting: Selecting<T>,
        pub updating: Updating<T>,
    }

    pub struct Selecting<T>
    where
        T: Sealed,
    {
        pub configuration: T,
    }

    pub struct Updating<T>
    where
        T: Sealed,
    {
        pub configuration: T,
    }

    pub struct String_(pub String);

    pub struct StringLiteral(pub &'static str);

    mod sealed {
        use super::{
            StringLiteral,
            String_,
        };

        pub trait Sealed {}

        impl Sealed for StringLiteral {}

        impl Sealed for String_ {}
    }

    pub(crate) mod environment_configuration_file {
        use serde::Deserialize;

        #[derive(Deserialize)]
        pub struct EnvironmentConfigurationFile {
            pub tokio_runtime: TokioRuntime,
            pub application_server: ApplicationServer,
            pub remote_service: RemoteService,
            pub logging: Logging,
            pub security: Security,
            pub resource: Resource,
        }

        #[derive(Deserialize)]
        pub struct TokioRuntime {
            pub maximum_blocking_threads_quantity: Value<usize>,
            pub worker_threads_quantity: Value<usize>,
            pub worker_thread_stack_size: Value<usize>,
        }

        #[derive(Deserialize)]
        pub struct ApplicationServer {
            pub tcp: Tcp,
            pub http: Http,
        }

        #[derive(Deserialize)]
        pub struct Tcp {
            pub socket_address: Value<String>,
            pub nodelay: Value<bool>,
            pub sleep_on_accept_errors: Value<bool>,
            pub keepalive: TcpKeepalive,
        }

        #[derive(Deserialize)]
        pub struct TcpKeepalive {
            pub duration: ValueExist<u64>,
            pub interval_duration: ValueExist<u64>,
            pub retries_quantity: ValueExist<u32>,
        }

        #[derive(Deserialize)]
        pub struct Http {
            pub adaptive_window: Value<bool>,
            pub connection_window_size: Value<u32>,
            pub stream_window_size: Value<u32>,
            pub maximum_frame_size: Value<u32>,
            pub maximum_sending_buffer_size: Value<u32>,
            pub enable_connect_protocol: Value<bool>,
            pub maximum_header_list_size: Value<u32>,
            pub maximum_pending_accept_reset_streams: ValueExist<usize>,
            pub keepalive: HttpKeepalive,
        }

        #[derive(Deserialize)]
        pub struct HttpKeepalive {
            pub is_exist: bool,
            pub interval_duration: Value<u64>,
            pub timeout_duration: Value<u64>,
        }

        #[derive(Deserialize)]
        pub struct RemoteService {
            pub user_authorization: UserAuthorization,
        }

        #[derive(Deserialize)]
        pub struct UserAuthorization {
            pub url: Value<String>,
        }

        #[derive(Deserialize)]
        pub struct Logging {
            pub directory_path: Value<String>,
            pub file_name_prefix: Value<String>,
        }

        #[derive(Deserialize)]
        pub struct Security {
            pub server_access_token: Value<String>,
        }

        #[derive(Deserialize)]
        pub struct Resource {
            pub clickhouse: Clickhouse,
            pub postgresql: Postgresql,
        }

        #[derive(Deserialize)]
        pub struct Clickhouse {
            pub url: Value<String>,
            pub user: Value<String>,
            pub password: Value<String>,
            pub database: Value<String>,
        }

        #[derive(Deserialize)]
        pub struct Postgresql {
            pub selecting: Selecting,
            pub updating: Updating,
        }

        #[derive(Deserialize)]
        pub struct Selecting {
            pub configuration: Value<String>,
        }

        #[derive(Deserialize)]
        pub struct Updating {
            pub configuration: Value<String>,
        }

        #[derive(Deserialize)]
        pub struct Value<T> {
            pub value: T,
        }

        #[derive(Deserialize)]
        pub struct ValueExist<T> {
            pub value: T,
            pub is_exist: bool,
        }
    }
}

pub mod error {
    use std::{
        convert::From,
        error::Error as StdError,
        fmt::{
            Display,
            Error as FormatError,
            Formatter,
        },
        io::Error as IOError,
    };
    use toml::de::Error as TomlError;

    #[derive(Debug)]
    pub enum Error {
        Logic {
            message: &'static str,
        },
        Other {
            other: Other,
        },
    }

    impl Display for Error {
        fn fmt<'a, 'b>(&'a self, formatter: &'b mut Formatter<'_>) -> Result<(), FormatError> {
            match *self {
                Self::Logic {
                    ref message,
                } => {
                    return write!(
                        formatter,
                        "Error, logic: {}.",
                        message
                    );
                }
                Self::Other {
                    ref other,
                } => {
                    return write!(
                        formatter,
                        "Error, other: {}.",
                        other.get_message()
                    );
                }
            }
        }
    }

    impl StdError for Error {}

    impl From<IOError> for Error {
        fn from(value: IOError) -> Self {
            return Self::Other {
                other: Other::new(value),
            };
        }
    }

    impl From<TomlError> for Error {
        fn from(value: TomlError) -> Self {
            return Self::Other {
                other: Other::new(value),
            };
        }
    }

    #[derive(Debug)]
    pub struct Other {
        message: String,
    }

    impl Other {
        pub fn new<E>(error: E) -> Self
        where
            E: StdError,
        {
            return Self {
                message: format!(
                    "{}",
                    error
                ),
            };
        }

        pub fn get_message<'a>(&'a self) -> &'a str {
            return self.message.as_str();
        }
    }

    impl Display for Other {
        fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FormatError> {
            return Ok(());
        }
    }
}

pub mod loader {
    use super::{
        environment_configuration::{
            environment_configuration_file::EnvironmentConfigurationFile,
            ApplicationServer,
            Clickhouse,
            Environment,
            EnvironmentConfiguration,
            Http,
            HttpKeepalive,
            Logging,
            Postgresql,
            RemoteService,
            Resource,
            Security,
            Selecting,
            String_,
            Tcp,
            TcpKeepalive,
            TokioRuntime,
            Updating,
            UserAuthorization,
        },
        error::Error,
    };
    use std::{
        fs::read_to_string,
        path::Path,
    };
    use toml::from_str;

    pub struct Loader;

    impl Loader {
        const PRODUCTION_ENVIRONMENT_DIRECTORY_NAME: &'static str = "production";
        const DEVELOPMENT_ENVIRONMENT_DIRECTORY_NAME: &'static str = "development";
        const LOCAL_DEVELOPMENT_ENVIRONMENT_DIRECTORY_NAME: &'static str = "local_development";
        const ENVIRONMENT_FILE_NAME: &'static str = "environment.toml";

        pub fn load_from_file<'a>(environment_configuration_directory_path: &'a str) -> Result<EnvironmentConfiguration<String_>, Error> {
            let production_environment_file_path = format!(
                "{}/{}/{}",
                environment_configuration_directory_path,
                Self::PRODUCTION_ENVIRONMENT_DIRECTORY_NAME,
                Self::ENVIRONMENT_FILE_NAME,
            );

            let production_environment_file_path_ = Path::new(production_environment_file_path.as_str());

            let (environment, environment_file_data) = if production_environment_file_path_.try_exists()? {
                (
                    Environment::Production,
                    read_to_string(production_environment_file_path_)?,
                )
            } else {
                let local_development_environment_file_path = format!(
                    "{}/{}/{}",
                    environment_configuration_directory_path,
                    Self::LOCAL_DEVELOPMENT_ENVIRONMENT_DIRECTORY_NAME,
                    Self::ENVIRONMENT_FILE_NAME,
                );

                let local_development_environment_file_path_ = Path::new(local_development_environment_file_path.as_str());

                if local_development_environment_file_path_.try_exists()? {
                    (
                        Environment::LocalDevelopment,
                        read_to_string(local_development_environment_file_path_)?,
                    )
                } else {
                    let development_environment_file_path = format!(
                        "{}/{}/{}",
                        environment_configuration_directory_path,
                        Self::DEVELOPMENT_ENVIRONMENT_DIRECTORY_NAME,
                        Self::ENVIRONMENT_FILE_NAME,
                    );

                    let development_environment_file_path_ = Path::new(development_environment_file_path.as_str());

                    if development_environment_file_path_.try_exists()? {
                        (
                            Environment::Development,
                            read_to_string(development_environment_file_path_)?,
                        )
                    } else {
                        return Err(
                            Error::Logic {
                                message: "The environment.toml file does not exist.",
                            },
                        );
                    }
                }
            };

            let environment_configuration_file = from_str::<EnvironmentConfigurationFile>(environment_file_data.as_str())?;

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
                            socket_address: String_(environment_configuration_file.application_server.tcp.socket_address.value),
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
                        user_authorization: UserAuthorization {
                            url: String_(environment_configuration_file.remote_service.user_authorization.url.value),
                        },
                    },
                    logging: Logging {
                        directory_path: String_(environment_configuration_file.logging.directory_path.value),
                        file_name_prefix: String_(environment_configuration_file.logging.file_name_prefix.value),
                    },
                    security: Security {
                        server_access_token: String_(environment_configuration_file.security.server_access_token.value),
                    },
                    resource: Resource {
                        clickhouse: Clickhouse {
                            url: String_(environment_configuration_file.resource.clickhouse.url.value),
                            user: String_(environment_configuration_file.resource.clickhouse.user.value),
                            password: String_(environment_configuration_file.resource.clickhouse.password.value),
                            database: String_(environment_configuration_file.resource.clickhouse.database.value),
                        },
                        postgresql: Postgresql {
                            selecting: Selecting {
                                configuration: String_(environment_configuration_file.resource.postgresql.selecting.configuration.value),
                            },
                            updating: Updating {
                                configuration: String_(environment_configuration_file.resource.postgresql.updating.configuration.value),
                            },
                        },
                    },
                }
            };

            return Ok(environment_configuration);
        }
    }
}

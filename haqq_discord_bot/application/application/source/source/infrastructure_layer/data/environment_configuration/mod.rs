pub mod environment_configuration_file;

pub struct EnvironmentConfiguration {
    pub remote_service: RemoteService,
    pub noncontext_parameters: NoncontextParameters,
    pub tokio_runtime: TokioRuntime,
    pub application_server: ApplicationServer,
    pub logging: Logging,
    pub security: Security,
    pub resource: Resource,
}

pub struct RemoteService {
    pub discord: Discord,
    pub haqq: Haqq,
}

pub struct Discord {
    pub guild: Guild,
    pub application: Application,
}

pub struct Guild {
    pub id: u64,
}

pub struct Application {
    pub bot: Bot,
}

pub struct Bot {
    pub id: u64,
    pub role_id: u64,
    pub public_key: String,
    pub token: String,
}

pub struct Haqq {
    pub evm_node: EvmNode,
}

pub struct EvmNode {
    pub url: String,
}

pub struct TokioRuntime {
    pub maximum_blocking_threads_quantity: usize,
    pub worker_threads_quantity: usize,
    pub worker_thread_stack_size: usize,
}

pub struct ApplicationServer {
    pub tcp: Tcp,
    pub http: Http,
}

pub struct Tcp {
    pub socket_address: String,
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

pub struct Logging {
    pub directory_path: String,
    pub file_name_prefix: String,
}

pub struct Security {
    pub server_access_token: String,
}

pub struct Resource {
    pub postgresql: Postgresql,
}

pub struct Postgresql {
    pub configuration: String,
}

pub struct NoncontextParameters {
    pub raffle_stake_updating_cron_configuration: String,
    pub aislm_stake_streshold_quantity_for_stakers_club_role: String,
    pub discord_roles_updating_cron_configuration: String,
    pub algorithm_repetition_in_error_case_quantity: usize,
    pub wallet_verification_process_duraction_minutes: i64,
}

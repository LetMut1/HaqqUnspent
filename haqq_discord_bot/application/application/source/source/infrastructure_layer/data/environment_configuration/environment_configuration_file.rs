use serde::Deserialize;

#[derive(Deserialize)]
pub struct EnvironmentConfigurationFile {
    pub remote_service: RemoteService,
    pub noncontext_parameters: NoncontextParameters,
    pub tokio_runtime: TokioRuntime,
    pub application_server: ApplicationServer,
    pub logging: Logging,
    pub security: Security,
    pub resource: Resource,
}

#[derive(Deserialize)]
pub struct RemoteService {
    pub discord: Discord,
    pub haqq: Haqq,
}

#[derive(Deserialize)]
pub struct Discord {
    pub guild: Guild,
    pub application: Application,
}

#[derive(Deserialize)]
pub struct Guild {
    pub id: Value<u64>,
}

#[derive(Deserialize)]
pub struct Application {
    pub bot: Bot,
}

#[derive(Deserialize)]
pub struct Bot {
    pub id: Value<u64>,
    pub role_id: Value<u64>,
    pub public_key: Value<String>,
    pub token: Value<String>,
}

#[derive(Deserialize)]
pub struct Haqq {
    pub evm_node: EvmNode,
}

#[derive(Deserialize)]
pub struct EvmNode {
    pub url: Value<String>,
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
    pub postgresql: Postgresql,
}

#[derive(Deserialize)]
pub struct Postgresql {
    pub configuration: Value<String>,
}

#[derive(Deserialize)]
pub struct NoncontextParameters {
    pub raffle_stake_updating_cron_configuration: Value<String>,
    pub aislm_stake_streshold_quantity_for_stakers_club_role: Value<String>,
    pub discord_roles_updating_cron_configuration: Value<String>,
    pub algorithm_repetition_in_error_case_quantity: Value<usize>,
    pub wallet_verification_process_duraction_minutes: Value<i64>,
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

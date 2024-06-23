use serde::Deserialize;

#[derive(Deserialize)]
pub struct EnvironmentConfigurationFile {
    pub tokio_runtime: TokioRuntime,
    pub application_server: ApplicationServer,
    pub remote_service: RemoteService,
    pub logging: Logging,
    pub security: Security,
    pub resource: Resource,
    pub task: Task,
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
    pub data_proxy: DataProxy,
    pub coingecko: Coingecko,
}

#[derive(Deserialize)]
pub struct DataProxy {
    pub url: Value<String>,
    pub server_access_token: Value<String>,
}

#[derive(Deserialize)]
pub struct Coingecko {
    pub pro: Pro,
}

#[derive(Deserialize)]
pub struct Pro {
    pub url: Value<String>,
    pub api_key: Value<String>,
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
pub struct Task {
    pub generate_aggregated_balance_snapshot: GenerateAggregatedBalanceSnapshot,
    pub generate_base_balance_snapshot: GenerateBaseBalanceSnapshot,
    pub update_assets___generate_asset_snapshot: UpdateAssets__GenerateAssetSnapshot,
    pub update_assets_for_subportfolio_trackable_wallet: UpdateAssetsForSubportfolioTrackableWallet,
}

#[derive(Deserialize)]
pub struct GenerateAggregatedBalanceSnapshot {
    pub cron_configuration: Value<String>,
}

#[derive(Deserialize)]
pub struct GenerateBaseBalanceSnapshot {
    pub cron_configuration: Value<String>,
}

#[derive(Deserialize)]
pub struct UpdateAssets__GenerateAssetSnapshot {
    pub cron_configuration: Value<String>,
}

#[derive(Deserialize)]
pub struct UpdateAssetsForSubportfolioTrackableWallet {
    pub cron_configuration: Value<String>,
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

pub mod environment_configuration_file;

pub enum Environment {
    Production,
    Development,
    LocalDevelopment,
}

pub struct EnvironmentConfiguration {
    pub environment: Environment,
    pub tokio_runtime: TokioRuntime,
    pub application_server: ApplicationServer,
    pub remote_service: RemoteService,
    pub logging: Logging,
    pub security: Security,
    pub resource: Resource,
    pub task: Task,
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

pub struct RemoteService {
    pub data_proxy: DataProxy,
    pub coingecko: Coingecko,
}

pub struct DataProxy {
    pub url: String,
    pub server_access_token: String,
}

pub struct Coingecko {
    pub pro: Pro,
}

pub struct Pro {
    pub url: String,
    pub api_key: String,
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

pub struct Task {
    pub generate_aggregated_balance_snapshot: GenerateAggregatedBalanceSnapshot,
    pub generate_base_balance_snapshot: GenerateBaseBalanceSnapshot,
    pub update_assets___generate_asset_snapshot: UpdateAssets__GenerateAssetSnapshot,
    pub update_assets_for_subportfolio_trackable_wallet: UpdateAssetsForSubportfolioTrackableWallet,
}

pub struct GenerateAggregatedBalanceSnapshot {
    pub cron_configuration: String,
}

pub struct GenerateBaseBalanceSnapshot {
    pub cron_configuration: String,
}

pub struct UpdateAssets__GenerateAssetSnapshot {
    pub cron_configuration: String,
}

pub struct UpdateAssetsForSubportfolioTrackableWallet {
    pub cron_configuration: String,
}

use super::Creator;
use crate::infrastructure_layer::data::{
    auditor::Auditor,
    backtrace::BacktracePart,
    error::{
        Error,
        Other,
        Runtime,
    },
};
pub use clickhouse::Client as ClickhouseClient;
use clickhouse::Compression;
use hyper::{
    client::connect::HttpConnector,
    Body,
    Client as HyperClient,
};
use hyper_tls::{
    native_tls::TlsConnector,
    HttpsConnector,
};
use std::time::Duration;

impl Creator<ClickhouseClient> {
    const POOL_IDLE_TIMEOUT: Duration = Duration::from_secs(30);

    pub fn create(url: &'static str, user: &'static str, password: &'static str, database: &'static str) -> Result<ClickhouseClient, Auditor<Error>> {
        let mut http_connector = HttpConnector::new();

        http_connector.enforce_http(false);

        let mut tls_connector_builder = TlsConnector::builder();

        tls_connector_builder.danger_accept_invalid_certs(true);

        let tls_connector = match tls_connector_builder.build() {
            Ok(tls_connector_) => tls_connector_,
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

        let https_connector = HttpsConnector::from(
            (
                http_connector,
                tls_connector.into(),
            ),
        );

        let hyper_client = HyperClient::builder().pool_idle_timeout(Self::POOL_IDLE_TIMEOUT).build::<_, Body>(https_connector);

        let clickhouse_client = ClickhouseClient::with_http_client(hyper_client)
            .with_url(url)
            .with_user(user)
            .with_password(password)
            .with_database(database)
            .with_compression(Compression::Lz4);

        return Ok(clickhouse_client);
    }
}

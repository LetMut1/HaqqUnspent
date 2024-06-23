use super::Creator;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use std::str::FromStr;
use tokio_postgres::config::Config;
use tokio_postgres::NoTls;

pub type PostgresqlConnectionPoolNoTls = Pool<PostgresConnectionManager<NoTls>>;

impl Creator<PostgresqlConnectionPoolNoTls> {
    pub async fn create<'a>(configuration: &'a str) -> Result<PostgresqlConnectionPoolNoTls, Auditor<Error>> {
        let configuration_ = match Config::from_str(configuration) {
            Ok(configuration__) => configuration__,
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

        let postgresql_connection_pool = match Pool::builder()
            .build(
                PostgresConnectionManager::new(
                    configuration_,
                    NoTls,
                ),
            )
            .await
        {
            Ok(postgresql_connection_pool_) => postgresql_connection_pool_,
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

        return Ok(postgresql_connection_pool);
    }
}

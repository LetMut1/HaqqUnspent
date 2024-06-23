use super::ClickhouseRepository;
use crate::infrastructure_layer::data::{
    auditor::Auditor,
    backtrace::BacktracePart,
    error::{
        Error,
        Other,
        Runtime,
    },
};
use clickhouse::{
    self,
    Client,
};

pub struct Common;

impl ClickhouseRepository<Common> {
    pub async fn check_health<'a>(clickhouse_client: &'a Client) -> Result<(), Auditor<Error>> {
        let query = "SELECT true";

        let query_ = clickhouse_client.query(query);

        let mut row_cursor = match query_.fetch::<bool>() {
            Ok(row_cursor_) => row_cursor_,
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

        if let Err(error) = row_cursor.next().await {
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
}

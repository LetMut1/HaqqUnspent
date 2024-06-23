use super::Creator;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        environment_configuration::EnvironmentConfiguration,
        error::{
            Error,
            Other,
            Runtime,
        },
    },
    functionality::service::spawner::{
        tokio_non_blocking_task::TokioNonBlockingTask,
        Spawner,
    },
};
use tokio_postgres::{
    connect,
    Client,
    NoTls,
};

impl Creator<Client> {
    pub async fn create<'a>(environment_configuration: &'a EnvironmentConfiguration) -> Result<Client, Auditor<Error>> {
        let (client, connection) = match connect(
            environment_configuration.resource.postgresql.configuration.as_str(),
            NoTls,
        )
        .await
        {
            Ok(tupple) => tupple,
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

        let future = async {
            if let Err(error) = connection.await {
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

            Ok(())
        };

        Spawner::<TokioNonBlockingTask>::spawn_into_background(future);

        return Ok(client);
    }
}

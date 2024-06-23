use super::PostgresqlRepository;
use crate::domain_layer::data::entity::address_verification_data::AddressVerificationData;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use tokio_postgres::types::ToSql;
use tokio_postgres::types::Type;
use tokio_postgres::Client;

impl PostgresqlRepository<AddressVerificationData> {
    pub async fn create<'a>(
        client: &'a Client,
        address_verification_data: &'a AddressVerificationData,
    ) -> Result<(), Auditor<Error>> {
        let query = "\
            INSERT INTO public.address_verification_data AS avd ( \
                discord_user_id, \
                recipient_bech32_address, \
                expected_token_quantity, \
                created_at, \
                expired_at \
            ) VALUES ( \
                $1, \
                $2, \
                $3, \
                $4, \
                $5 \
            );";

        let statement = match client
            .prepare_typed(
                query,
                [
                    Type::TEXT,
                    Type::TEXT,
                    Type::TEXT,
                    Type::INT8,
                    Type::INT8,
                ]
                .as_slice(),
            )
            .await
        {
            Ok(statement_) => statement_,
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

        if let Err(error) = client
            .query(
                &statement,
                ([
                    &address_verification_data.discord_user_id,
                    &address_verification_data.recipient_bech32_address,
                    &address_verification_data.expected_token_quantity,
                    &address_verification_data.created_at,
                    &address_verification_data.expired_at,
                ] as [&(dyn ToSql + Sync); 5])
                    .as_slice(),
            )
            .await
        {
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

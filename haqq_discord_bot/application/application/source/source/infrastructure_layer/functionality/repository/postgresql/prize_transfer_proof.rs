use super::PostgresqlRepository;
use crate::domain_layer::data::entity::prize_transfer_proof::PrizeTransferProof;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use tokio_postgres::types::ToSql;
use tokio_postgres::types::Type;
use tokio_postgres::Client;

impl PostgresqlRepository<PrizeTransferProof> {
    pub async fn batch_insert<'a>(
        client: &'a Client,
        prize_transfer_proof_registry: &'a [PrizeTransferProof],
    ) -> Result<(), Auditor<Error>> {
        if prize_transfer_proof_registry.is_empty() {
            return Ok(());
        }

        let length = prize_transfer_proof_registry.len();

        let mut type_registry: Vec<Type> = vec![];

        let mut value_registry: Vec<&(dyn ToSql + Sync)> = vec![];

        let mut counter = 0 as usize;

        let mut query = "\
            INSERT INTO public.prize_transfer_proof AS ptp (\
                raffle_id, \
                discord_user_id, \
                evm_transaction_hash, \
                created_at\
            ) VALUES"
            .to_string();

        '_a: for (index, prize_transfer_proof) in prize_transfer_proof_registry.iter().enumerate() {
            counter += 1;

            let value_1 = counter;

            counter += 1;

            let value_2 = counter;

            counter += 1;

            let value_3 = counter;

            counter += 1;

            let value_4 = counter;

            query = format!(
                "{} \
                (\
                    ${}, \
                    ${}, \
                    ${}, \
                    ${}\
                )",
                query.as_str(),
                value_1,
                value_2,
                value_3,
                value_4,
            );

            if index < (length - 1) {
                query = format!(
                    "{},",
                    query.as_str()
                );
            } else {
                query = format!(
                    "{};",
                    query.as_str()
                );
            }

            type_registry.push(Type::INT8);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::INT8);

            value_registry.push(&prize_transfer_proof.raffle_id);

            value_registry.push(&prize_transfer_proof.discord_user_id);

            value_registry.push(&prize_transfer_proof.evm_transaction_hash);

            value_registry.push(&prize_transfer_proof.created_at);
        }

        let statement = match client
            .prepare_typed(
                query.as_str(),
                type_registry.as_slice(),
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
                value_registry.as_slice(),
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

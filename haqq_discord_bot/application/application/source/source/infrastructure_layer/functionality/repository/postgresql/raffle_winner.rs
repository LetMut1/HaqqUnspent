use super::PostgresqlRepository;
use crate::domain_layer::data::entity::raffle_winner::RaffleWinner;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use tokio_postgres::types::ToSql;
use tokio_postgres::types::Type;
use tokio_postgres::Client;

impl PostgresqlRepository<RaffleWinner> {
    pub async fn batch_insert<'a>(
        client: &'a Client,
        raffle_winner_registry: &'a [RaffleWinner],
    ) -> Result<(), Auditor<Error>> {
        if raffle_winner_registry.is_empty() {
            return Ok(());
        }

        let length = raffle_winner_registry.len();

        let mut type_registry: Vec<Type> = vec![];

        let mut value_registry: Vec<&(dyn ToSql + Sync)> = vec![];

        let mut counter = 0 as usize;

        let mut query = "\
            INSERT INTO public.raffle_winner AS rf (\
                raffle_id, \
                discord_user_id, \
                bech32_address, \
                created_at\
            ) VALUES"
            .to_string();

        '_a: for (index, raffle_winner) in raffle_winner_registry.iter().enumerate() {
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
            }

            type_registry.push(Type::INT8);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::INT8);

            value_registry.push(&raffle_winner.raffle_id);

            value_registry.push(&raffle_winner.discord_user_id);

            value_registry.push(&raffle_winner.bech32_address);

            value_registry.push(&raffle_winner.created_at);
        }

        query = format!(
            "{} \
            ON CONFLICT(raffle_id, discord_user_id) \
            DO UPDATE SET \
                bech32_address = EXCLUDED.bech32_address;",
            query.as_str()
        );

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

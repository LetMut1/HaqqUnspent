use super::by::By_6;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::raffle_participant::RaffleParticipant;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use tokio_postgres::types::ToSql;
use tokio_postgres::types::Type;
use tokio_postgres::Client;

impl PostgresqlRepository<RaffleParticipant> {
    pub async fn create<'a>(
        client: &'a Client,
        raffle_participant: &'a RaffleParticipant,
    ) -> Result<(), Auditor<Error>> {
        let query = "\
            INSERT INTO public.raffle_participant AS rp (\
                raffle_id, \
                discord_user_id, \
                created_at\
            ) VALUES (\
                $1, \
                $2, \
                $3\
            );";

        let statement = match client
            .prepare_typed(
                query,
                [
                    Type::INT8,
                    Type::TEXT,
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
                    &raffle_participant.raffle_id,
                    &raffle_participant.discord_user_id,
                    &raffle_participant.created_at,
                ] as [&(dyn ToSql + Sync); 3])
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

    pub async fn delete<'a>(
        client: &'a Client,
        by_6: &'a By_6<'_>,
    ) -> Result<(), Auditor<Error>> {
        let query = "\
            DELETE FROM ONLY public.raffle_participant AS rp \
            WHERE \
                rp.raffle_id = $1 \
                AND rp.discord_user_id = $2;";

        let statement = match client
            .prepare_typed(
                query,
                [
                    Type::INT8,
                    Type::TEXT,
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
                    &by_6.raffle_participant_raffle_id,
                    &by_6.raffle_participant_discord_user_id,
                ] as [&(dyn ToSql + Sync); 2])
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
        }

        return Ok(());
    }

    pub async fn is_exist<'a>(
        client: &'a Client,
        by_6: &'a By_6<'_>,
    ) -> Result<bool, Auditor<Error>> {
        let query = "\
            SELECT \
                COUNT(*) as c \
            FROM \
                public.raffle_participant rp \
            WHERE \
                rp.raffle_id = $1 \
                AND rp.discord_user_id = $2;";

        let statement = match client
            .prepare_typed(
                query,
                [
                    Type::INT8,
                    Type::TEXT,
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

        let row_registry = match client
            .query(
                &statement,
                ([
                    &by_6.raffle_participant_raffle_id,
                    &by_6.raffle_participant_discord_user_id,
                ] as [&(dyn ToSql + Sync); 2])
                    .as_slice(),
            )
            .await
        {
            Ok(row_registry_) => row_registry_,
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

        if row_registry.is_empty() {
            return Err(
                Auditor::<Error>::new(
                    Error::create_value_does_not_exist(),
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        let count = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(count_) => count_,
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

        return Ok(count > 0);
    }
}

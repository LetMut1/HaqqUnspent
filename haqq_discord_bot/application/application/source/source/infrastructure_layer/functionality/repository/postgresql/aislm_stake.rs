use super::by::By_9;
use super::insert::Insert_3;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::aislm_stake::AislmStake;
use crate::domain_layer::data::entity::aislm_stake::AislmStake_1;
use crate::domain_layer::data::entity::aislm_stake::AislmStake_2;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use tokio_postgres::types::ToSql;
use tokio_postgres::types::Type;
use tokio_postgres::Client;

impl PostgresqlRepository<AislmStake> {
    pub async fn batch_upsert<'a>(
        client: &'a Client,
        insert_3_registry: &'a [Insert_3],
    ) -> Result<(), Auditor<Error>> {
        if insert_3_registry.is_empty() {
            return Ok(());
        }

        let length = insert_3_registry.len();

        let mut type_registry: Vec<Type> = vec![];

        let mut value_registry: Vec<&(dyn ToSql + Sync)> = vec![];

        let mut counter = 0 as usize;

        let mut query = "\
            INSERT INTO public.aislm_stake AS as_ (\
                id, \
                amount, \
                raffle_id, \
                bech32_address, \
                discord_user_id, \
                created_at\
            ) VALUES"
            .to_string();

        '_a: for (index, insert_3) in insert_3_registry.iter().enumerate() {
            counter += 1;

            let value_1 = counter;

            counter += 1;

            let value_2 = counter;

            counter += 1;

            let value_3 = counter;

            counter += 1;

            let value_4 = counter;

            counter += 1;

            let value_5 = counter;

            query = format!(
                "{} \
                (\
                    nextval('public.aislm_stake_1'), \
                    ${}, \
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
                value_5,
            );

            if index < (length - 1) {
                query = format!(
                    "{},",
                    query.as_str()
                );
            }

            type_registry.push(Type::TEXT);

            type_registry.push(Type::INT8);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::INT8);

            value_registry.push(&insert_3.aislm_stake_amount);

            value_registry.push(&insert_3.aislm_stake_raffle_id);

            value_registry.push(&insert_3.aislm_stake_bech32_address);

            value_registry.push(&insert_3.aislm_stake_discord_user_id);

            value_registry.push(&insert_3.aislm_stake_created_at);
        }

        query = format!(
            "{} \
            ON CONFLICT(raffle_id, created_at, discord_user_id, bech32_address) \
            DO UPDATE SET \
                amount = EXCLUDED.amount;",
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

impl PostgresqlRepository<AislmStake_1> {
    pub async fn get_all<'a>(
        client: &'a Client,
        by_9: By_9,
        aislm_stake_id: Option<i64>,
        limit: i16,
    ) -> Result<Vec<AislmStake_1>, Auditor<Error>> {
        let mut query = format!(
            "SELECT \
                as_.id AS i, \
                as_.amount AS a, \
                as_.bech32_address AS ba, \
                as_.discord_user_id AS dui, \
                as_.created_at AS ca \
            FROM \
                public.aislm_stake as_ \
            WHERE \
                as_.raffle_id = {}",
            by_9.aislm_stake_raffle_id,
        );

        if let Some(aislm_stake_id_) = aislm_stake_id {
            query = format!(
                "{} \
                AND as_.id > {}",
                query.as_str(),
                aislm_stake_id_,
            );
        }

        query = format!(
            "{} \
            ORDER BY \
                as_.id ASC \
            LIMIT {};",
            query.as_str(),
            limit,
        );

        let row_registry = match client
            .query(
                query.as_str(),
                ([] as [&(dyn ToSql + Sync); 0]).as_slice(),
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

        let mut aislm_stake_1_registry: Vec<AislmStake_1> = vec![];

        '_a: for row in row_registry.into_iter() {
            let aislm_stake_id = match row.try_get::<'_, usize, i64>(0) {
                Ok(aislm_stake_id_) => aislm_stake_id_,
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

            let aislm_stake_amount = match row.try_get::<'_, usize, String>(1) {
                Ok(aislm_stake_amount_) => aislm_stake_amount_,
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

            let aislm_stake_bech32_address = match row.try_get::<'_, usize, String>(2) {
                Ok(aislm_stake_bech32_address_) => aislm_stake_bech32_address_,
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

            let aislm_stake_discord_user_id = match row.try_get::<'_, usize, String>(3) {
                Ok(aislm_stake_discord_user_id_) => aislm_stake_discord_user_id_,
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

            let aislm_stake_created_at = match row.try_get::<'_, usize, i64>(4) {
                Ok(aislm_stake_created_at_) => aislm_stake_created_at_,
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

            aislm_stake_1_registry.push(
                AislmStake_1 {
                    id: aislm_stake_id,
                    amount: aislm_stake_amount,
                    discord_user_id: aislm_stake_discord_user_id,
                    bech32_address: aislm_stake_bech32_address,
                    created_at: aislm_stake_created_at,
                },
            );
        }

        return Ok(aislm_stake_1_registry);
    }
}

impl PostgresqlRepository<AislmStake_2> {
    pub async fn get_all<'a>(
        client: &'a Client,
        by_9: By_9,
        aislm_stake_id: Option<i64>,
        limit: i16,
    ) -> Result<Vec<AislmStake_2>, Auditor<Error>> {
        let mut query = format!(
            "SELECT \
                as_.id AS i, \
                as_.amount AS a, \
                as_.discord_user_id AS dui \
            FROM \
                public.aislm_stake as_ \
            WHERE \
                as_.raffle_id = {} \
                AND as_.created_at = (\
                    SELECT \
                        MAX(as_.created_at) as ca \
                    FROM \
                        public.aislm_stake as_ \
                    WHERE \
                        as_.raffle_id = {}\
                )",
            by_9.aislm_stake_raffle_id, by_9.aislm_stake_raffle_id,
        );

        if let Some(aislm_stake_id_) = aislm_stake_id {
            query = format!(
                "{} \
                AND as_.id > {}",
                query.as_str(),
                aislm_stake_id_,
            )
        }

        let query = format!(
            "{} \
            ORDER BY \
                as_.id ASC \
            LIMIT {};",
            query.as_str(),
            limit,
        );

        let row_registry = match client
            .query(
                query.as_str(),
                ([] as [&(dyn ToSql + Sync); 0]).as_slice(),
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

        let mut aislm_stake_2_registry: Vec<AislmStake_2> = vec![];

        '_a: for row in row_registry.into_iter() {
            let aislm_stake_id = match row.try_get::<'_, usize, i64>(0) {
                Ok(aislm_stake_id_) => aislm_stake_id_,
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

            let aislm_stake_amount = match row.try_get::<'_, usize, String>(1) {
                Ok(aislm_stake_amount_) => aislm_stake_amount_,
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

            let aislm_stake_discord_user_id = match row.try_get::<'_, usize, String>(2) {
                Ok(aislm_stake_discord_user_id_) => aislm_stake_discord_user_id_,
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

            aislm_stake_2_registry.push(
                AislmStake_2 {
                    id: aislm_stake_id,
                    amount: aislm_stake_amount,
                    discord_user_id: aislm_stake_discord_user_id,
                },
            );
        }

        return Ok(aislm_stake_2_registry);
    }
}

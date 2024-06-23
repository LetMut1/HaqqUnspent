use super::by::By_3;
use super::by::By_5;
use super::by::By_8;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::verified_bech32_address::VerifiedBech32Address;
use crate::domain_layer::data::entity::verified_bech32_address::VerifiedBech32Address_1;
use crate::domain_layer::data::entity::verified_bech32_address::VerifiedBech32Address_2;
use crate::domain_layer::data::entity::verified_bech32_address::VerifiedBech32Address_3;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use tokio_postgres::types::ToSql;
use tokio_postgres::types::Type;
use tokio_postgres::Client;

impl PostgresqlRepository<VerifiedBech32Address> {
    pub async fn create<'a>(
        client: &'a Client,
        verified_bech32_address: &'a VerifiedBech32Address,
    ) -> Result<(), Auditor<Error>> {
        let query = "\
            INSERT INTO public.verified_bech32_address AS vba ( \
                value, \
                discord_user_id, \
                created_at
            ) VALUES ( \
                $1, \
                $2, \
                $3 \
            );";

        let statement = match client
            .prepare_typed(
                query,
                [
                    Type::TEXT,
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
                    &verified_bech32_address.value,
                    &verified_bech32_address.discord_user_id,
                    &verified_bech32_address.created_at,
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
}

impl PostgresqlRepository<VerifiedBech32Address_1> {
    // The Limit+Offset functionality is not needed here, since in a real case
    // the probability that the user will link at least 100 addresses is very low.
    pub async fn get_all_available<'a>(
        client: &'a Client,
        by_3: &'a By_3<'_>,
    ) -> Result<Vec<VerifiedBech32Address_1>, Auditor<Error>> {
        let query = "\
            SELECT \
                vba.value AS v \
            FROM \
                public.verified_bech32_address vba \
            LEFT OUTER JOIN \
                public.verified_address_blacklist vab \
            ON \
                vba.value = vab.bech32_address \
            WHERE \
                vba.discord_user_id = $1 \
                AND vab.bech32_address IS NULL;";

        let statement = match client
            .prepare_typed(
                query,
                [Type::TEXT].as_slice(),
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
                ([&by_3.verified_bech32_address_discord_user_id] as [&(dyn ToSql + Sync); 1]).as_slice(),
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

        let mut verified_bech32_address_1_registry: Vec<VerifiedBech32Address_1> = vec![];

        '_a: for row in row_registry.into_iter() {
            let verified_bech32_address_value = match row.try_get::<'_, usize, String>(0) {
                Ok(verified_bech32_address_value_) => verified_bech32_address_value_,
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

            verified_bech32_address_1_registry.push(
                VerifiedBech32Address_1 {
                    value: verified_bech32_address_value,
                },
            );
        }

        return Ok(verified_bech32_address_1_registry);
    }
}

impl PostgresqlRepository<VerifiedBech32Address_2> {
    pub async fn find<'a>(
        client: &'a Client,
        by_5: &'a By_5<'_>,
    ) -> Result<Option<VerifiedBech32Address_2>, Auditor<Error>> {
        let query = "\
            SELECT \
                vba.discord_user_id AS dui \
            FROM \
                public.verified_bech32_address vba \
            WHERE \
                vba.value = $1";

        let statement = match client
            .prepare_typed(
                query,
                [Type::TEXT].as_slice(),
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
                ([&by_5.verified_bech32_address_value] as [&(dyn ToSql + Sync); 1]).as_slice(),
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
            return Ok(None);
        }

        let verified_bech32_address_discord_user_id = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(verified_bech32_address_discord_user_id_) => verified_bech32_address_discord_user_id_,
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

        return Ok(
            Some(
                VerifiedBech32Address_2 {
                    discord_user_id: verified_bech32_address_discord_user_id,
                },
            ),
        );
    }
}

impl PostgresqlRepository<VerifiedBech32Address_3> {
    pub async fn get_all_available_without_role<'a>(
        client: &'a Client,
        verified_bech32_address_discord_user_id: Option<&'a str>,
        limit: i16,
    ) -> Result<
        Vec<(
            String,
            Vec<String>,
        )>,
        Auditor<Error>,
    > {
        let mut query = "\
            SELECT \
                vba.discord_user_id AS dui, \
                ARRAY_AGG(vba.value) AS aav \
            FROM \
                public.verified_bech32_address vba \
            INNER JOIN \
                (\
                    SELECT \
                        dur.discord_user_id AS dui \
                    FROM \
                        public.discord_user_role dur \
                    WHERE \
                        dur.stakers_club_member = FALSE\
                ) AS dur \
            ON \
                vba.discord_user_id = dur.dui \
            LEFT OUTER JOIN \
                public.verified_address_blacklist vab \
            ON \
                vba.value = vab.bech32_address \
            WHERE \
                vab.bech32_address IS NULL"
            .to_string();

        if let Some(_) = verified_bech32_address_discord_user_id {
            query = format!(
                "{} \
                AND vba.discord_user_id > $1",
                query.as_str(),
            );
        }

        query = format!(
            "{} \
            GROUP BY \
                vba.discord_user_id \
            ORDER BY \
                vba.discord_user_id ASC \
            LIMIT {};",
            query.as_str(),
            limit,
        );

        let row_registry = match verified_bech32_address_discord_user_id {
            Some(verified_bech32_address_discord_user_id_) => {
                let statement = match client
                    .prepare_typed(
                        query.as_str(),
                        [Type::TEXT].as_slice(),
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
                        ([&verified_bech32_address_discord_user_id_] as [&(dyn ToSql + Sync); 1]).as_slice(),
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

                row_registry
            }
            None => {
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

                row_registry
            }
        };

        let mut registry: Vec<(
            String,
            Vec<String>,
        )> = vec![];

        '_a: for row in row_registry.into_iter() {
            let verified_bech32_address_discord_user_id = match row.try_get::<'_, usize, String>(0) {
                Ok(verified_bech32_address_discord_user_id_) => verified_bech32_address_discord_user_id_,
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

            let verified_bech32_address_value_registry = match row.try_get::<'_, usize, Vec<String>>(1) {
                Ok(verified_bech32_address_value_registry_) => verified_bech32_address_value_registry_,
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

            registry.push(
                (
                    verified_bech32_address_discord_user_id,
                    verified_bech32_address_value_registry,
                ),
            );
        }

        return Ok(registry);
    }

    pub async fn get_all_available_for_raffle<'a>(
        client: &'a Client,
        by_8: By_8,
        verified_bech32_address_discord_user_id: Option<&'a str>,
        limit: i16,
    ) -> Result<
        Vec<(
            String,
            Vec<String>,
        )>,
        Auditor<Error>,
    > {
        let mut query = format!(
            "SELECT \
                vba.discord_user_id AS dui, \
                ARRAY_AGG(vba.value) AS aav \
            FROM \
                public.verified_bech32_address vba \
            INNER JOIN \
                (\
                    SELECT \
                        rp.discord_user_id AS dui \
                    FROM \
                        public.raffle_participant rp \
                    WHERE \
                        rp.raffle_id = {}\
                ) AS rp \
            ON \
                vba.discord_user_id = rp.dui \
            LEFT OUTER JOIN \
                public.verified_address_blacklist vab \
            ON \
                vba.value = vab.bech32_address \
            WHERE \
                vab.bech32_address IS NULL",
            by_8.raffle_id,
        );

        if let Some(_) = verified_bech32_address_discord_user_id {
            query = format!(
                "{} \
                AND vba.discord_user_id > $1",
                query.as_str(),
            );
        }

        query = format!(
            "{} \
            GROUP BY \
                vba.discord_user_id \
            ORDER BY \
                vba.discord_user_id ASC \
            LIMIT {};",
            query.as_str(),
            limit,
        );

        let row_registry = match verified_bech32_address_discord_user_id {
            Some(verified_bech32_address_discord_user_id_) => {
                let statement = match client
                    .prepare_typed(
                        query.as_str(),
                        [Type::TEXT].as_slice(),
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
                        ([&verified_bech32_address_discord_user_id_] as [&(dyn ToSql + Sync); 1]).as_slice(),
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

                row_registry
            }
            None => {
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

                row_registry
            }
        };

        let mut registry: Vec<(
            String,
            Vec<String>,
        )> = vec![];

        '_a: for row in row_registry.into_iter() {
            let verified_bech32_address_discord_user_id = match row.try_get::<'_, usize, String>(0) {
                Ok(verified_bech32_address_discord_user_id_) => verified_bech32_address_discord_user_id_,
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

            let verified_bech32_address_value_registry = match row.try_get::<'_, usize, Vec<String>>(1) {
                Ok(verified_bech32_address_value_registry_) => verified_bech32_address_value_registry_,
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

            registry.push(
                (
                    verified_bech32_address_discord_user_id,
                    verified_bech32_address_value_registry,
                ),
            );
        }

        return Ok(registry);
    }
}

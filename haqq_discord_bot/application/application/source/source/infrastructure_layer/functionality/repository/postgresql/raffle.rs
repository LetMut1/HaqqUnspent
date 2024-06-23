use super::by::By_8;
use super::insert::Insert_2;
use super::update::Update_4;
use super::update::Update_5;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::raffle::Raffle_5;
use crate::domain_layer::data::entity::raffle::Raffle_Status;
use crate::domain_layer::data::entity::raffle::Raffle;
use crate::domain_layer::data::entity::raffle::Raffle_1;
use crate::domain_layer::data::entity::raffle::Raffle_2;
use crate::domain_layer::data::entity::raffle::Raffle_3;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use tokio_postgres::types::ToSql;
use tokio_postgres::types::Type;
use tokio_postgres::Client;

impl PostgresqlRepository<Raffle> {
    pub async fn create<'a>(
        client: &'a Client,
        insert_2: Insert_2,
    ) -> Result<Raffle, Auditor<Error>> {
        let query = "\
            INSERT INTO public.raffle AS r (\
                id, \
                islm_prize_amount, \
                winners_number, \
                seed, \
                aes_key, \
                status, \
                created_at, \
                expired_at\
            ) VALUES (\
                nextval('public.raffle_1'), \
                $1, \
                $2, \
                $3, \
                $4, \
                $5, \
                $6, \
                $7\
            ) \
            RETURNING \
                r.id AS i;";

        let statement = match client
            .prepare_typed(
                query,
                [
                    Type::INT8,
                    Type::INT8,
                    Type::TEXT,
                    Type::TEXT,
                    Type::INT2,
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

        let row_registry = match client
            .query(
                &statement,
                ([
                    &insert_2.raffle_islm_prize_amount,
                    &insert_2.raffle_winners_number,
                    &insert_2.raffle_seed,
                    &insert_2.raffle_aes_key,
                    &insert_2.raffle_status.to_representation(),
                    &insert_2.raffle_created_at,
                    &insert_2.raffle_expired_at,
                ] as [&(dyn ToSql + Sync); 7])
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
                    Error::create_unreachable_state(),
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        let raffle_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(raffle_id_) => raffle_id_,
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
            Raffle::new(
                raffle_id,
                insert_2.raffle_islm_prize_amount,
                insert_2.raffle_winners_number,
                insert_2.raffle_seed,
                insert_2.raffle_aes_key,
                insert_2.raffle_status,
                insert_2.raffle_created_at,
                insert_2.raffle_expired_at,
            ),
        );
    }

    pub async fn is_exist_in_progress<'a>(client: &'a Client) -> Result<bool, Auditor<Error>> {
        let query = format!(
            "SELECT \
                COUNT(*) as c \
            FROM \
                public.raffle r \
            WHERE \
                r.status = {} \
                OR r.status = {};",
                Raffle_Status::ParticipantsRecruitment.to_representation(),
                Raffle_Status::PrizeTransfer.to_representation(),
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

    pub async fn find_in_progress<'a>(client: &'a Client) -> Result<Option<Raffle>, Auditor<Error>> {
        let query = format!(
            "SELECT \
                r.id AS i, \
                r.islm_prize_amount AS ipa, \
                r.winners_number AS wn, \
                r.seed AS se, \
                r.aes_key AS ak, \
                r.status AS st, \
                r.created_at AS ca, \
                r.expired_at AS ea \
            FROM \
                public.raffle r \
            WHERE \
                r.status = {} \
                OR r.status = {};",
                Raffle_Status::ParticipantsRecruitment.to_representation(),
                Raffle_Status::PrizeTransfer.to_representation(),
        );

        let row_registry = match client
            .query(
                query.as_str(),
                &([] as [&(dyn ToSql + Sync); 0]).as_slice(),
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

        let raffle_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(raffle_id_) => raffle_id_,
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

        let raffle_islm_prize_amount = match row_registry[0].try_get::<'_, usize, i64>(1) {
            Ok(raffle_islm_prize_amount_) => raffle_islm_prize_amount_,
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

        let raffle_winners_number = match row_registry[0].try_get::<'_, usize, i64>(2) {
            Ok(raffle_winners_number_) => raffle_winners_number_,
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

        let raffle_seed = match row_registry[0].try_get::<'_, usize, String>(3) {
            Ok(raffle_seed_) => raffle_seed_,
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

        let raffle_aes_key = match row_registry[0].try_get::<'_, usize, String>(4) {
            Ok(raffle_aes_key_) => raffle_aes_key_,
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

        let raffle_status = match row_registry[0].try_get::<'_, usize, i16>(5) {
            Ok(raffle_status_) => raffle_status_,
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

        let raffle_created_at = match row_registry[0].try_get::<'_, usize, i64>(6) {
            Ok(raffle_created_at_) => raffle_created_at_,
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

        let raffle_expired_at = match row_registry[0].try_get::<'_, usize, i64>(7) {
            Ok(raffle_expired_at_) => raffle_expired_at_,
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
                Raffle::new(
                    raffle_id,
                    raffle_islm_prize_amount,
                    raffle_winners_number,
                    raffle_seed,
                    raffle_aes_key,
                    Raffle_Status::from_representation(raffle_status),
                    raffle_created_at,
                    raffle_expired_at,
                ),
            ),
        );
    }

    pub async fn find<'a>(
        client: &'a Client,
        by_8: By_8,
    ) -> Result<Option<Raffle>, Auditor<Error>> {
        let query = format!(
            "SELECT \
                r.islm_prize_amount AS ipa, \
                r.winners_number AS wn, \
                r.seed AS se, \
                r.aes_key AS ak, \
                r.status AS st, \
                r.created_at AS ca, \
                r.expired_at AS ea \
            FROM \
                public.raffle r \
            WHERE \
                r.id = {};",
                by_8.raffle_id,
        );

        let row_registry = match client
            .query(
                query.as_str(),
                &([] as [&(dyn ToSql + Sync); 0]).as_slice(),
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

        let raffle_islm_prize_amount = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(raffle_islm_prize_amount_) => raffle_islm_prize_amount_,
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

        let raffle_winners_number = match row_registry[0].try_get::<'_, usize, i64>(1) {
            Ok(raffle_winners_number_) => raffle_winners_number_,
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

        let raffle_seed = match row_registry[0].try_get::<'_, usize, String>(2) {
            Ok(raffle_seed_) => raffle_seed_,
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

        let raffle_aes_key = match row_registry[0].try_get::<'_, usize, String>(3) {
            Ok(raffle_aes_key_) => raffle_aes_key_,
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

        let raffle_status = match row_registry[0].try_get::<'_, usize, i16>(4) {
            Ok(raffle_status_) => raffle_status_,
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

        let raffle_created_at = match row_registry[0].try_get::<'_, usize, i64>(5) {
            Ok(raffle_created_at_) => raffle_created_at_,
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

        let raffle_expired_at = match row_registry[0].try_get::<'_, usize, i64>(6) {
            Ok(raffle_expired_at_) => raffle_expired_at_,
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
                Raffle::new(
                    by_8.raffle_id,
                    raffle_islm_prize_amount,
                    raffle_winners_number,
                    raffle_seed,
                    raffle_aes_key,
                    Raffle_Status::from_representation(raffle_status),
                    raffle_created_at,
                    raffle_expired_at,
                ),
            ),
        );
    }
}

impl PostgresqlRepository<Raffle_1> {
    pub async fn find_in_progress<'a>(client: &'a Client) -> Result<Option<Raffle_1>, Auditor<Error>> {
        let query = format!(
            "SELECT \
                r.id as i, \
                r.expired_at AS ea \
            FROM \
                public.raffle r \
            WHERE \
                r.status = {} \
                OR r.status = {};",
                Raffle_Status::ParticipantsRecruitment.to_representation(),
                Raffle_Status::PrizeTransfer.to_representation(),
        );

        let row_registry = match client
            .query(
                query.as_str(),
                &([] as [&(dyn ToSql + Sync); 0]).as_slice(),
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

        let raffle_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(raffle_id_) => raffle_id_,
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

        let raffle_expired_at = match row_registry[0].try_get::<'_, usize, i64>(1) {
            Ok(raffle_expired_at_) => raffle_expired_at_,
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
                Raffle_1 {
                    id: raffle_id,
                    expired_at: raffle_expired_at,
                },
            ),
        );
    }
}

impl PostgresqlRepository<Raffle_2> {
    pub async fn update<'a>(
        client: &'a Client,
        update_4: Update_4,
        by_8: By_8,
    ) -> Result<(), Auditor<Error>> {
        let query = format!(
            "UPDATE ONLY public.raffle r \
            SET (\
                status\
            ) = ROW(\
                {}\
            ) \
            WHERE \
                r.id = {};",
            update_4.raffle_status.to_representation(),
            by_8.raffle_id,
        );

        if let Err(error) = client
            .query(
                query.as_str(),
                ([] as [&(dyn ToSql + Sync); 0]).as_slice(),
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

impl PostgresqlRepository<Raffle_3> {
    pub async fn find<'a>(
        client: &'a Client,
        by_8: By_8,
    ) -> Result<Option<Raffle_3>, Auditor<Error>> {
        let query = format!(
            "SELECT \
                r.status as s, \
                r.expired_at \
            FROM \
                public.raffle r \
            WHERE \
                r.id = {};",
            by_8.raffle_id,
        );

        let row_registry = match client
            .query(
                query.as_str(),
                &([] as [&(dyn ToSql + Sync); 0]).as_slice(),
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

        let raffle_status = match row_registry[0].try_get::<'_, usize, i16>(0) {
            Ok(raffle_status_) => raffle_status_,
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

        let raffle_expired_at = match row_registry[0].try_get::<'_, usize, i64>(1) {
            Ok(raffle_expired_at_) => raffle_expired_at_,
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
                Raffle_3 {
                    status: Raffle_Status::from_representation(raffle_status),
                    expired_at: raffle_expired_at,
                },
            ),
        );
    }
}

impl PostgresqlRepository<Raffle_5> {
    pub async fn update<'a>(
        client: &'a Client,
        update_5: Update_5,
        by_8: By_8,
    ) -> Result<(), Auditor<Error>> {
        let query = format!(
            "UPDATE ONLY public.raffle r \
            SET (\
                islm_prize_amount, \
                winners_number, \
                expired_at\
            ) = ROW(\
                {}, \
                {}, \
                {}\
            ) \
            WHERE \
                r.id = {};",
            update_5.raffle_islm_prize_amount,
            update_5.raffle_winners_number,
            update_5.raffle_expired_at,
            by_8.raffle_id,
        );

        if let Err(error) = client
            .query(
                query.as_str(),
                ([] as [&(dyn ToSql + Sync); 0]).as_slice(),
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
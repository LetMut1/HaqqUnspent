use super::by::By_7;
use super::update::Update_1;
use super::update::Update_2;
use super::update::Update_3;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::discord_user_role::DiscordUserRole;
use crate::domain_layer::data::entity::discord_user_role::DiscordUserRole_1;
use crate::domain_layer::data::entity::discord_user_role::DiscordUserRole_2;
use crate::domain_layer::data::entity::discord_user_role::DiscordUserRole_3;
use crate::domain_layer::data::entity::discord_user_role::DiscordUserRole_4;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use tokio_postgres::types::ToSql;
use tokio_postgres::types::Type;
use tokio_postgres::Client;

impl PostgresqlRepository<DiscordUserRole> {
    pub async fn create<'a>(
        client: &'a Client,
        discord_user_role: &'a DiscordUserRole,
    ) -> Result<(), Auditor<Error>> {
        let query = "\
            INSERT INTO public.discord_user_role AS dur (\
                discord_user_id, \
                wallet_verified, \
                stakers_club_member, \
                updated_at\
            ) VALUES (\
                $1, \
                $2, \
                $3, \
                $4\
            );";

        let statement = match client
            .prepare_typed(
                query,
                [
                    Type::TEXT,
                    Type::BOOL,
                    Type::BOOL,
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
                    &discord_user_role.discord_user_id,
                    &discord_user_role.wallet_verified,
                    &discord_user_role.stakers_club_member,
                    &discord_user_role.updated_at,
                ] as [&(dyn ToSql + Sync); 4])
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

impl PostgresqlRepository<DiscordUserRole_1> {
    pub async fn find<'a>(
        client: &'a Client,
        by_7: &'a By_7<'_>,
    ) -> Result<Option<DiscordUserRole_1>, Auditor<Error>> {
        let query = "\
            SELECT \
                dur.wallet_verified, \
                dur.stakers_club_member \
            FROM \
                public.discord_user_role dur \
            WHERE \
                dur.discord_user_id = $1;";

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
                ([&by_7.discord_user_role_discord_user_id] as [&(dyn ToSql + Sync); 1]).as_slice(),
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

        let discord_user_role_wallet_verified = match row_registry[0].try_get::<'_, usize, bool>(0) {
            Ok(discord_user_role_wallet_verified_) => discord_user_role_wallet_verified_,
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

        let discord_user_role_stakers_club_member = match row_registry[0].try_get::<'_, usize, bool>(1) {
            Ok(discord_user_role_stakers_club_member_) => discord_user_role_stakers_club_member_,
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
                DiscordUserRole_1 {
                    wallet_verified: discord_user_role_wallet_verified,
                    stakers_club_member: discord_user_role_stakers_club_member,
                },
            ),
        );
    }
}

impl PostgresqlRepository<DiscordUserRole_2> {
    pub async fn update<'a>(
        client: &'a Client,
        update_1: Update_1,
        by_7: &'a By_7<'_>,
    ) -> Result<(), Auditor<Error>> {
        let query = "\
            UPDATE ONLY public.discord_user_role dur \
            SET (\
                wallet_verified, \
                stakers_club_member, \
                updated_at\
            ) = ROW( \
                $1, \
                $2, \
                $3\
            ) \
            WHERE \
                dur.discord_user_id = $4;";

        let statement = match client
            .prepare_typed(
                query,
                [
                    Type::BOOL,
                    Type::BOOL,
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
                    &update_1.discord_user_role_wallet_verified,
                    &update_1.discord_user_role_stakers_club_member,
                    &update_1.discord_user_role_updated_at,
                    &by_7.discord_user_role_discord_user_id,
                ] as [&(dyn ToSql + Sync); 4])
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

impl PostgresqlRepository<DiscordUserRole_3> {
    pub async fn update<'a>(
        client: &'a Client,
        update_2: Update_2,
        by_7: &'a By_7<'_>,
    ) -> Result<(), Auditor<Error>> {
        let query = "\
            UPDATE ONLY public.discord_user_role dur \
            SET (\
                wallet_verified, \
                updated_at\
            ) = ROW( \
                $1, \
                $2\
            ) \
            WHERE \
                dur.discord_user_id = $3;";

        let statement = match client
            .prepare_typed(
                query,
                [
                    Type::BOOL,
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
                    &update_2.discord_user_role_wallet_verified,
                    &update_2.discord_user_role_updated_at,
                    &by_7.discord_user_role_discord_user_id,
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

impl PostgresqlRepository<DiscordUserRole_4> {
    pub async fn update<'a>(
        client: &'a Client,
        update_3: Update_3,
        by_7: &'a By_7<'_>,
    ) -> Result<(), Auditor<Error>> {
        let query = "\
            UPDATE ONLY public.discord_user_role dur \
            SET (\
                stakers_club_member, \
                updated_at\
            ) = ROW( \
                $1, \
                $2\
            ) \
            WHERE \
                dur.discord_user_id = $3;";

        let statement = match client
            .prepare_typed(
                query,
                [
                    Type::BOOL,
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
                    &update_3.discord_user_role_stakers_club_member,
                    &update_3.discord_user_role_updated_at,
                    &by_7.discord_user_role_discord_user_id,
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

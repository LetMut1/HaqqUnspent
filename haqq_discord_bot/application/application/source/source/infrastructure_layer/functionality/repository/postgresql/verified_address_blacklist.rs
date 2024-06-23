use super::by::By_4;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::verified_address_blacklist::VerifiedAddressBlacklist;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use tokio_postgres::types::ToSql;
use tokio_postgres::types::Type;
use tokio_postgres::Client;

impl PostgresqlRepository<VerifiedAddressBlacklist> {
    pub async fn create<'a>(
        client: &'a Client,
        verified_address_blacklist: &'a VerifiedAddressBlacklist,
    ) -> Result<(), Auditor<Error>> {
        let query = "\
            INSERT INTO public.verified_address_blacklist AS vab ( \
                bech32_address, \
                created_at
            ) VALUES ( \
                $1, \
                $2 \
            );";

        let statement = match client
            .prepare_typed(
                query,
                [
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
                    &verified_address_blacklist.bech32_address,
                    &verified_address_blacklist.created_at,
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
        };

        return Ok(());
    }

    pub async fn delete<'a>(
        client: &'a Client,
        by_4: &'a By_4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let query = "\
            DELETE FROM ONLY \
                public.verified_address_blacklist AS vab \
            WHERE \
                vab.bech32_address = $1";

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

        if let Err(error) = client
            .query(
                &statement,
                ([&by_4.verified_address_blacklist_bech32_address] as [&(dyn ToSql + Sync); 1]).as_slice(),
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

    pub async fn is_exist<'a>(
        client: &'a Client,
        by_4: &'a By_4<'_>,
    ) -> Result<bool, Auditor<Error>> {
        let query = "\
            SELECT \
                COUNT(*) as c \
            FROM \
                public.verified_address_blacklist vab \
            WHERE \
                vab.bech32_address = $1;";

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
                ([&by_4.verified_address_blacklist_bech32_address] as [&(dyn ToSql + Sync); 1]).as_slice(),
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

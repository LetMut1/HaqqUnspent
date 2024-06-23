use super::by::By_10;
use super::insert::Insert_4;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::sender_hd_wallet::SenderHdWallet;
use crate::domain_layer::data::entity::sender_hd_wallet::SenderHdWallet_1;
use crate::domain_layer::data::entity::sender_hd_wallet::SenderHdWallet_2;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use tokio_postgres::types::ToSql;
use tokio_postgres::types::Type;
use tokio_postgres::Client;

impl PostgresqlRepository<SenderHdWallet> {
    pub async fn create<'a>(
        client: &'a Client,
        insert_4: Insert_4,
    ) -> Result<SenderHdWallet, Auditor<Error>> {
        let query = "\
            INSERT INTO public.sender_hd_wallet AS shw ( \
                id, \
                mnemonic_phrase, \
                mnemonic_derevation_path_index, \
                created_at \
            ) VALUES ( \
                nextval('public.sender_hd_wallet_1'), \
                $1, \
                $2, \
                $3 \
            ) \
            RETURNING \
                shw.id AS i;";

        let statement = match client
            .prepare_typed(
                query,
                [
                    Type::TEXT,
                    Type::INT4,
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
                    &insert_4.sender_hd_wallet_mnemonic_phrase,
                    &insert_4.sender_hd_wallet_mnemonic_derevation_path_index,
                    &insert_4.sender_hd_wallet_created_at,
                ] as [&(dyn ToSql + Sync); 3])
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

        let sender_hd_wallet_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(sender_hd_wallet_id_) => sender_hd_wallet_id_,
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
            SenderHdWallet::new(
                sender_hd_wallet_id,
                insert_4.sender_hd_wallet_mnemonic_phrase,
                insert_4.sender_hd_wallet_mnemonic_derevation_path_index,
                insert_4.sender_hd_wallet_created_at,
            ),
        );
    }
}

impl PostgresqlRepository<SenderHdWallet_1> {
    pub async fn find<'a>(client: &'a Client) -> Result<Option<SenderHdWallet_1>, Auditor<Error>> {
        let query = "\
            SELECT \
                MAX(shw.id) AS i \
            FROM \
                public.sender_hd_wallet shw;";

        let row_registry = match client
            .query(
                query,
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

        let sender_hd_wallet_id = match row_registry[0].try_get::<'_, usize, Option<i64>>(0) {
            Ok(sender_hd_wallet_id_) => sender_hd_wallet_id_,
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

        let sender_hd_wallet_1 = match sender_hd_wallet_id {
            Some(sender_hd_wallet_id_) => Some(
                SenderHdWallet_1 {
                    id: sender_hd_wallet_id_,
                },
            ),
            None => None,
        };

        return Ok(sender_hd_wallet_1);
    }
}

impl PostgresqlRepository<SenderHdWallet_2> {
    pub async fn get<'a>(
        client: &'a Client,
        by_10: By_10,
    ) -> Result<SenderHdWallet_2, Auditor<Error>> {
        let query = format!(
            "\
            SELECT \
                shw.mnemonic_phrase AS mp, \
                shw.mnemonic_derevation_path_index AS mdpi \
            FROM \
                public.sender_hd_wallet shw \
            WHERE \
                shw.id = {};",
            by_10.sender_hd_wallet_id,
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

        let sender_hd_wallet_mnemonic_phrase = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(sender_hd_wallet_mnemonic_phrase_) => sender_hd_wallet_mnemonic_phrase_,
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

        let sender_hd_wallet_mnemonic_derevation_path_index = match row_registry[0].try_get::<'_, usize, i32>(1) {
            Ok(sender_hd_wallet_mnemonic_derevation_path_index_) => sender_hd_wallet_mnemonic_derevation_path_index_,
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
            SenderHdWallet_2 {
                mnemonic_phrase: sender_hd_wallet_mnemonic_phrase,
                mnemonic_derevation_path_index: sender_hd_wallet_mnemonic_derevation_path_index,
            },
        );
    }
}
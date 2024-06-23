use super::by::By_1;
use super::insert::Insert_1;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::recipient_hd_wallet::RecipientHdWallet;
use crate::domain_layer::data::entity::recipient_hd_wallet::RecipientHdWallet_1;
use crate::domain_layer::data::entity::recipient_hd_wallet::RecipientHdWallet_2;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use tokio_postgres::types::ToSql;
use tokio_postgres::types::Type;
use tokio_postgres::Client;

impl PostgresqlRepository<RecipientHdWallet> {
    pub async fn create<'a>(
        client: &'a Client,
        insert_1: Insert_1,
    ) -> Result<RecipientHdWallet, Auditor<Error>> {
        let query = "\
            INSERT INTO public.recipient_hd_wallet AS rhw ( \
                id, \
                mnemonic_phrase, \
                mnemonic_derevation_path_index, \
                created_at \
            ) VALUES ( \
                nextval('public.recipient_hd_wallet_1'), \
                $1, \
                $2, \
                $3 \
            ) \
            RETURNING \
                rhw.id AS i;";

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
                    &insert_1.recipient_hd_wallet_mnemonic_phrase,
                    &insert_1.recipient_hd_wallet_mnemonic_derevation_path_index,
                    &insert_1.recipient_hd_wallet_created_at,
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

        let recipient_hd_wallet_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(recipient_hd_wallet_id_) => recipient_hd_wallet_id_,
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
            RecipientHdWallet::new(
                recipient_hd_wallet_id,
                insert_1.recipient_hd_wallet_mnemonic_phrase,
                insert_1.recipient_hd_wallet_mnemonic_derevation_path_index,
                insert_1.recipient_hd_wallet_created_at,
            ),
        );
    }

    pub async fn increment_mnemonic_derevation_path_index<'a>(
        client: &'a Client,
        by_1: By_1,
    ) -> Result<RecipientHdWallet_2, Auditor<Error>> {
        let query = format!(
            "\
            UPDATE ONLY public.recipient_hd_wallet AS rhw
            SET \
                mnemonic_derevation_path_index = mnemonic_derevation_path_index + 1 \
            WHERE \
                rhw.id = {} \
            RETURNING \
                rhw.mnemonic_phrase AS mp,
                rhw.mnemonic_derevation_path_index AS mdpi;",
            by_1.recipient_hd_wallet_id,
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

        let recipient_hd_wallet_mnemonic_phrase = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(recipient_hd_wallet_mnemonic_phrase_) => recipient_hd_wallet_mnemonic_phrase_,
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

        let recipient_hd_wallet_mnemonic_derevation_path_index = match row_registry[0].try_get::<'_, usize, i32>(1) {
            Ok(recipient_hd_wallet_mnemonic_derevation_path_index_) => recipient_hd_wallet_mnemonic_derevation_path_index_,
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
            RecipientHdWallet_2 {
                mnemonic_phrase: recipient_hd_wallet_mnemonic_phrase,
                mnemonic_derevation_path_index: recipient_hd_wallet_mnemonic_derevation_path_index,
            },
        );
    }
}

impl PostgresqlRepository<RecipientHdWallet_1> {
    pub async fn find<'a>(client: &'a Client) -> Result<Option<RecipientHdWallet_1>, Auditor<Error>> {
        let query = "\
            SELECT \
                MAX(rhw.id) AS i \
            FROM \
                public.recipient_hd_wallet rhw;";

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

        let recipient_hd_wallet_id = match row_registry[0].try_get::<'_, usize, Option<i64>>(0) {
            Ok(recipient_hd_wallet_id_) => recipient_hd_wallet_id_,
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

        let recipient_hd_wallet_1 = match recipient_hd_wallet_id {
            Some(recipient_hd_wallet_id_) => Some(
                RecipientHdWallet_1 {
                    id: recipient_hd_wallet_id_,
                },
            ),
            None => None,
        };

        return Ok(recipient_hd_wallet_1);
    }
}

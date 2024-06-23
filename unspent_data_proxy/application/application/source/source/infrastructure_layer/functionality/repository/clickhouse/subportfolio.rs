use super::{
    by::{
        By2,
        By3,
        By7,
    },
    ClickhouseRepository,
};
use crate::{
    domain_layer::data::entity::subportfolio::{
        Subportfolio,
        Subportfolio_1,
        Subportfolio_2,
        Subportfolio_CreatedAt,
        Subportfolio_Description,
        Subportfolio_Id,
        Subportfolio_Name,
    },
    infrastructure_layer::data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        error::{
            Error,
            Other,
            Runtime,
        },
    },
};
use clickhouse::{
    self,
    Client,
    Row,
};
use serde::{
    Deserialize,
    Serialize,
};

impl ClickhouseRepository<Subportfolio> {
    pub async fn create<'a>(clickhouse_client: &'a Client, subportfolio: &'a Subportfolio) -> Result<(), Auditor<Error>> {
        let mut query = "\
            INSERT INTO unspentio.subportfolio \
            (\
                id, \
                user_id, \
                name, \
                description, \
                created_at, \
                updated_at, \
                is_deleted\
            ) \
            VALUES \
            (\
                ?, \
                ?, \
                ?, "
        .to_string();

        match subportfolio.description {
            Some(_) => {
                query = format!(
                    "{}\
                        ?, ",
                    query.as_str(),
                )
            }
            None => {
                query = format!(
                    "{}\
                        NULL, ",
                    query.as_str(),
                )
            }
        }

        query = format!(
            "{}\
                fromUnixTimestamp(?), \
                fromUnixTimestamp(?), \
                ?\
            )",
            query.as_str(),
        );

        let mut query_ = clickhouse_client.query(query.as_str()).bind(&subportfolio.id).bind(subportfolio.user_id).bind(&subportfolio.name);

        if let Some(ref description) = subportfolio.description {
            query_ = query_.bind(description);
        }

        query_ = query_.bind(subportfolio.created_at).bind(subportfolio.updated_at).bind(subportfolio.is_deleted);

        if let Err(error) = query_.execute().await {
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

    pub async fn get_count<'a>(clickhouse_client: &'a Client, by_2: By2) -> Result<u64, Auditor<Error>> {
        let query = format!(
            "SELECT \
                COUNT(*) AS c \
            FROM \
                unspentio.subportfolio s \
            FINAL \
            WHERE \
                s.user_id = {} \
            SETTINGS \
                optimize_move_to_prewhere = 0",
            by_2.user_id
        );

        let query_ = clickhouse_client.query(query.as_str());

        let mut row_cursor = match query_.fetch::<u64>() {
            Ok(row_cursor_) => row_cursor_,
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

        let count = match row_cursor.next().await {
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

        let count_ = match count {
            Some(count__) => count__,
            None => {
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
        };

        return Ok(count_);
    }

    pub async fn is_exist_1<'a>(clickhouse_client: &'a Client, by_3: &'a By3<'_>) -> Result<bool, Auditor<Error>> {
        let query = "\
            SELECT \
                COUNT(*) AS c \
            FROM \
                unspentio.subportfolio s \
            FINAL \
            WHERE \
                s.user_id = ? \
                AND s.id = ? \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let query_ = clickhouse_client.query(query).bind(by_3.user_id).bind(by_3.subportfolio_id);

        let mut row_cursor = match query_.fetch::<u64>() {
            Ok(row_cursor_) => row_cursor_,
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

        let count = match row_cursor.next().await {
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

        let count_ = match count {
            Some(count__) => count__,
            None => {
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
        };

        let is_exist = count_ > 0;

        return Ok(is_exist);
    }

    pub async fn is_exist_2<'a>(clickhouse_client: &'a Client, by_7: &'a By7<'_>) -> Result<bool, Auditor<Error>> {
        let query = "\
            SELECT \
                COUNT(*) AS c \
            FROM \
                unspentio.subportfolio s \
            FINAL \
            WHERE \
                s.user_id = ? \
                AND s.name = ? \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let query_ = clickhouse_client.query(query).bind(by_7.user_id).bind(by_7.subportfolio_name);

        let mut row_cursor = match query_.fetch::<u64>() {
            Ok(row_cursor_) => row_cursor_,
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

        let count = match row_cursor.next().await {
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

        let count_ = match count {
            Some(count__) => count__,
            None => {
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
        };

        let is_exist = count_ > 0;

        return Ok(is_exist);
    }
}

impl ClickhouseRepository<Subportfolio_1> {
    pub async fn get_all<'a>(clickhouse_client: &'a Client, by_2: By2) -> Result<Vec<Subportfolio_1>, Auditor<Error>> {
        #[derive(Row, Serialize, Deserialize)]
        struct Subportfolio_ {
            id: String,
            name: String,
            description: String,
            is_null_description: u8,
        }

        let query = format!(
            "SELECT \
                s.id AS i, \
                s.name AS n, \
                COALESCE(s.description, '') AS d, \
                isNull(s.description) AS ind \
            FROM \
                unspentio.subportfolio s \
            FINAL \
            WHERE \
                s.user_id = {} \
            SETTINGS \
                optimize_move_to_prewhere = 0",
            by_2.user_id
        );

        let query_ = clickhouse_client.query(query.as_str());

        let mut row_cursor = match query_.fetch::<Subportfolio_>() {
            Ok(row_cursor_) => row_cursor_,
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

        let mut subportfolio_1_registry: Vec<Subportfolio_1> = vec![];

        'a: loop {
            let subportfolio = match row_cursor.next().await {
                Ok(subportfolio_) => subportfolio_,
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

            let subportfolio_ = match subportfolio {
                Some(subportfolio__) => subportfolio__,
                None => {
                    break 'a;
                }
            };

            let subportfolio_description = if subportfolio_.is_null_description == 1 {
                None
            } else {
                Some(Subportfolio_Description(subportfolio_.description))
            };

            subportfolio_1_registry.push(
                Subportfolio_1 {
                    id: Subportfolio_Id(subportfolio_.id),
                    name: Subportfolio_Name(subportfolio_.name),
                    description: subportfolio_description,
                },
            );
        }

        return Ok(subportfolio_1_registry);
    }
}

impl ClickhouseRepository<Subportfolio_2> {
    pub async fn find<'a>(clickhouse_client: &'a Client, by_3: &'a By3<'_>) -> Result<Option<Subportfolio_2>, Auditor<Error>> {
        #[derive(Row, Serialize, Deserialize)]
        struct Subportfolio_ {
            name: String,
            description: String,
            is_null_description: u8,
            created_at: u32,
        }

        let query = "\
            SELECT \
                s.name AS n, \
                COALESCE(s.description, '') AS d, \
                isNull(s.description) AS ind, \
                s.created_at AS ca \
            FROM \
                unspentio.subportfolio s \
            FINAL \
            WHERE \
                s.user_id = ? \
                AND s.id = ? \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let query_ = clickhouse_client.query(query).bind(by_3.user_id).bind(by_3.subportfolio_id);

        let mut row_cursor = match query_.fetch::<Subportfolio_>() {
            Ok(row_cursor_) => row_cursor_,
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

        let subportfolio = match row_cursor.next().await {
            Ok(subportfolio_) => subportfolio_,
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

        let subportfolio_ = match subportfolio {
            Some(subportfolio__) => subportfolio__,
            None => {
                return Ok(None);
            }
        };

        let subportfolio_description = if subportfolio_.is_null_description == 1 {
            None
        } else {
            Some(Subportfolio_Description(subportfolio_.description))
        };

        let subportfolio_2 = Subportfolio_2 {
            name: Subportfolio_Name(subportfolio_.name),
            description: subportfolio_description,
            created_at: Subportfolio_CreatedAt(subportfolio_.created_at),
        };

        return Ok(Some(subportfolio_2));
    }
}

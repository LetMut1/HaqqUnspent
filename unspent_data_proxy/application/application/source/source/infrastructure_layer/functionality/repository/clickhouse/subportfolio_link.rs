use super::{
    by::{
        By3,
        By8,
        By9,
    },
    ClickhouseRepository,
};
use crate::{
    domain_layer::data::entity::subportfolio_link::{
        SubportfolioLink,
        SubportfolioLink_1,
        SubportfolioLink_2,
        SubportfolioLink_3,
        SubportfolioLink_4,
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

impl ClickhouseRepository<SubportfolioLink> {
    pub async fn create<'a>(clickhouse_client: &'a Client, subportfolio_link: &'a SubportfolioLink) -> Result<(), Auditor<Error>> {
        let mut query = "\
            INSERT INTO unspentio.subportfolio_link \
            (\
                id, \
                user_id, \
                subportfolio_id, \
                is_active, \
                description, \
                created_at, \
                updated_at, \
                is_deleted\
            ) \
            VALUES \
            (\
                ?, \
                ?, \
                ?, \
                ?, "
        .to_string();

        match subportfolio_link.description {
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

        let mut query_ = clickhouse_client
            .query(query.as_str())
            .bind(subportfolio_link.id.as_str())
            .bind(subportfolio_link.user_id)
            .bind(subportfolio_link.subportfolio_id.as_str())
            .bind(subportfolio_link.is_active);

        if let Some(ref description) = subportfolio_link.description {
            query_ = query_.bind(description.as_str());
        }

        query_ = query_.bind(subportfolio_link.created_at).bind(subportfolio_link.updated_at).bind(subportfolio_link.is_deleted);

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

    pub async fn get_count<'a>(clickhouse_client: &'a Client, by_3: &'a By3<'_>) -> Result<u64, Auditor<Error>> {
        let query = "\
            SELECT \
                COUNT(*) AS c \
            FROM \
                unspentio.subportfolio_link sl \
            FINAL \
            WHERE \
                sl.user_id = ? \
                AND sl.subportfolio_id = ? \
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

        return Ok(count_);
    }

    pub async fn is_exist_1<'a>(clickhouse_client: &'a Client, by_8: &'a By8<'_>) -> Result<bool, Auditor<Error>> {
        let query = "\
            SELECT \
                COUNT(*) AS c \
            FROM \
                unspentio.subportfolio_link sl \
            FINAL \
            WHERE \
                sl.id = ? \
                AND sl.user_id = ? \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let query_ = clickhouse_client.query(query).bind(by_8.subportfolio_link_id).bind(by_8.user_id);

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

impl ClickhouseRepository<SubportfolioLink_1> {
    pub async fn get_all<'a>(clickhouse_client: &'a Client, by_3: &'a By3<'_>) -> Result<Vec<SubportfolioLink_1>, Auditor<Error>> {
        #[derive(Row, Serialize, Deserialize)]
        struct SubportfolioLink_ {
            id: String,
            is_active: bool,
            description: String,
            is_null_description: u8,
            created_at: u32,
        }

        let query = "\
            SELECT \
                sl.id AS i, \
                sl.is_active AS ia, \
                COALESCE(sl.description, '') AS d, \
                isNull(sl.description) AS ind, \
                sl.created_at AS ca \
            FROM \
                unspentio.subportfolio_link sl \
            FINAL \
            WHERE \
                sl.user_id = ? \
                AND sl.subportfolio_id = ? \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let query_ = clickhouse_client.query(query).bind(by_3.user_id).bind(by_3.subportfolio_id);

        let mut row_cursor = match query_.fetch::<SubportfolioLink_>() {
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

        let mut subportfolio_link_1_registry: Vec<SubportfolioLink_1> = vec![];

        'a: loop {
            let subportfolio_link = match row_cursor.next().await {
                Ok(subportfolio_link_) => subportfolio_link_,
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

            let subportfolio_link_ = match subportfolio_link {
                Some(subportfolio__) => subportfolio__,
                None => {
                    break 'a;
                }
            };

            let subportfolio_link_description = if subportfolio_link_.is_null_description == 1 {
                None
            } else {
                Some(subportfolio_link_.description)
            };

            subportfolio_link_1_registry.push(
                SubportfolioLink_1 {
                    id: subportfolio_link_.id,
                    is_active: subportfolio_link_.is_active,
                    description: subportfolio_link_description,
                    created_at: subportfolio_link_.created_at,
                },
            );
        }

        return Ok(subportfolio_link_1_registry);
    }
}

impl ClickhouseRepository<SubportfolioLink_2> {
    pub async fn find<'a>(clickhouse_client: &'a Client, by_8: &'a By8<'_>) -> Result<Option<SubportfolioLink_2>, Auditor<Error>> {
        #[derive(Row, Serialize, Deserialize)]
        struct SubportfolioLink_ {
            subportfolio_id: String,
            is_active: bool,
            description: String,
            is_null_description: u8,
            created_at: u32,
        }

        let query = "\
            SELECT \
                sl.subportfolio_id AS si, \
                sl.is_active AS ia, \
                COALESCE(sl.description, '') AS d, \
                isNull(sl.description) AS ind, \
                sl.created_at AS ca \
            FROM \
                unspentio.subportfolio_link sl \
            FINAL \
            WHERE \
                sl.id = ? \
                AND sl.user_id = ? \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let query_ = clickhouse_client.query(query).bind(by_8.subportfolio_link_id).bind(by_8.user_id);

        let mut row_cursor = match query_.fetch::<SubportfolioLink_>() {
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

        let subportfolio_link = match row_cursor.next().await {
            Ok(subportfolio_link_) => subportfolio_link_,
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

        let subportfolio_link_2 = match subportfolio_link {
            Some(subportfolio_link_) => {
                let subportfolio_link_description = if subportfolio_link_.is_null_description == 1 {
                    None
                } else {
                    Some(subportfolio_link_.description)
                };

                SubportfolioLink_2 {
                    subportfolio_id: subportfolio_link_.subportfolio_id,
                    is_active: subportfolio_link_.is_active,
                    description: subportfolio_link_description,
                    created_at: subportfolio_link_.created_at,
                }
            }
            None => {
                return Ok(None);
            }
        };

        return Ok(Some(subportfolio_link_2));
    }
}

impl ClickhouseRepository<SubportfolioLink_3> {
    pub async fn find<'a>(clickhouse_client: &'a Client, by_9: &'a By9<'_>) -> Result<Option<SubportfolioLink_3>, Auditor<Error>> {
        let query = "\
            SELECT \
                sl.user_id AS ui, \
                sl.subportfolio_id AS si, \
                sl.is_active AS ia \
            FROM \
                unspentio.subportfolio_link sl \
            FINAL \
            WHERE \
                sl.id = ? \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let query_ = clickhouse_client.query(query).bind(by_9.subportfolio_link_id);

        let mut row_cursor = match query_.fetch::<SubportfolioLink_3>() {
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

        let subportfolio_link_3 = match row_cursor.next().await {
            Ok(subportfolio_link_3_) => subportfolio_link_3_,
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

        return Ok(subportfolio_link_3);
    }
}

impl ClickhouseRepository<SubportfolioLink_4> {
    pub async fn find<'a>(clickhouse_client: &'a Client, by_9: &'a By9<'_>) -> Result<Option<SubportfolioLink_4>, Auditor<Error>> {
        let query = "\
            SELECT \
                sl.is_active AS ia \
            FROM \
                unspentio.subportfolio_link sl \
            FINAL \
            WHERE \
                sl.id = ? \
            SETTINGS \
                optimize_move_to_prewhere = 0";

        let query_ = clickhouse_client.query(query).bind(by_9.subportfolio_link_id);

        let mut row_cursor = match query_.fetch::<SubportfolioLink_4>() {
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

        let subportfolio_link_4 = match row_cursor.next().await {
            Ok(subportfolio_link_4_) => subportfolio_link_4_,
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

        return Ok(subportfolio_link_4);
    }
}

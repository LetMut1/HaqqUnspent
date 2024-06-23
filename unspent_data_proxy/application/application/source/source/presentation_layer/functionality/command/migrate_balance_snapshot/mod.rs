#![allow(unreachable_code)]

use application::infrastructure_layer::{
    data::environment_configuration::ENVIRONMENT_CONFIGURATION,
    functionality::service::creator::Creator,
};
use clickhouse::Client as ClickhouseClient;
use std::{
    error::Error,
    thread::sleep,
    time::Duration,
};
use tokio::{
    runtime::Builder,
    select,
};
use tokio_postgres::{
    connect,
    Client as PostgresqlCLient,
    NoTls,
};

const QUANTITY_OF_SECONDS_FOR_SLEEPING: u64 = 30;
const LIMIT: i64 = 15000;

fn main() -> () {
    'a: loop {
        match process() {
            Ok(_) => {
                println!("Finished.");

                return ();
            }
            Err(error) => {
                println!(
                    "ERROR: {}",
                    &error
                );

                println!(
                    "Process will sleep for {} seconds.",
                    QUANTITY_OF_SECONDS_FOR_SLEEPING
                );

                sleep(Duration::from_secs(QUANTITY_OF_SECONDS_FOR_SLEEPING));

                continue 'a;
            }
        }
    }

    return ();
}

pub fn process() -> Result<(), Box<dyn Error + 'static>> {
    let runtime = Builder::new_multi_thread().enable_all().build()?;

    runtime.block_on(migrate())?;

    return Ok(());
}

async fn migrate() -> Result<(), Box<dyn Error + 'static>> {
    let clickhouse_client = Creator::<ClickhouseClient>::create(
        ENVIRONMENT_CONFIGURATION.resource.clickhouse.url.0,
        ENVIRONMENT_CONFIGURATION.resource.clickhouse.user.0,
        ENVIRONMENT_CONFIGURATION.resource.clickhouse.password.0,
        ENVIRONMENT_CONFIGURATION.resource.clickhouse.database.0,
    )?;

    let (postgresql_client_updater, postgresql_connection_updater) = connect(
        ENVIRONMENT_CONFIGURATION.resource.postgresql.updating.configuration.0,
        NoTls,
    )
    .await?;

    let (postgresql_client_selector, postgresql_connection_selector) = connect(
        ENVIRONMENT_CONFIGURATION.resource.postgresql.selecting.configuration.0,
        NoTls,
    )
    .await?;

    select! {
        result = postgresql_connection_selector => {
            if let Err(error) = result {
                return Err(error.into());
            }

            ()
        },
        result = postgresql_connection_updater => {
            if let Err(error) = result {
                return Err(error.into());
            }

            ()
        },
        result = migrate_(
            &clickhouse_client,
            &postgresql_client_updater,
            &postgresql_client_selector,
        ) => {
            return result;
        }
    };

    return Ok(());
}

async fn migrate_<'a>(
    clickhouse_client: &'a ClickhouseClient,
    postgresql_client_updater: &'a PostgresqlCLient,
    postgresql_client_selector: &'a PostgresqlCLient,
) -> Result<(), Box<dyn Error + 'static>> {
    'a: loop {
        sleep(Duration::from_secs(1));

        let row_registry1 = postgresql_client_updater
            .query(
                "\
                SELECT \
                    bsg.last_inserted_id \
                FROM \
                    public.balance_snapshot_guard bsg \
                WHERE \
                    bsg.id = 1;",
                &[],
            )
            .await?;

        if row_registry1.is_empty() {
            let row_registry2 = postgresql_client_selector
                .query(
                    "\
                    SELECT \
                        min(bs.id) \
                    FROM \
                        public.balances_snapshots bs;",
                    &[],
                )
                .await?;

            if row_registry2.is_empty() {
                return Ok(());
            }

            let mut id = row_registry2[0].try_get::<'_, usize, i64>(0)?;

            id -= 1;

            match change_state(
                clickhouse_client,
                postgresql_client_updater,
                postgresql_client_selector,
                id,
            )
            .await
            {
                Ok(should_finish) => {
                    if should_finish {
                        break 'a;
                    }
                }
                Err(error) => {
                    return Err(error);
                }
            }
        } else {
            let id = row_registry1[0].try_get::<'_, usize, i64>(0)?;

            match change_state(
                clickhouse_client,
                postgresql_client_updater,
                postgresql_client_selector,
                id,
            )
            .await
            {
                Ok(should_finish) => {
                    if should_finish {
                        break 'a;
                    }
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
    }

    return Ok(());
}

async fn change_state<'a>(
    clickhouse_client: &'a ClickhouseClient,
    postgresql_client_updater: &'a PostgresqlCLient,
    postgresql_client_selector: &'a PostgresqlCLient,
    id: i64,
) -> Result<bool, Box<dyn Error + 'static>> {
    println!(
        "Data request with \'id\' = {} in progress.",
        id,
    );

    let row_registry3 = postgresql_client_selector
        .query(
            "\
            SELECT \
                bs.user_id, \
                bs.asset_id, \
                CASE \
                    WHEN bs.timestamp IS NOT NULL \
                    THEN to_char(bs.timestamp, 'YYYY-MM-DD HH24:MI:SS') \
                    ELSE NULL \
                END, \
                bs.total_balance::TEXT, \
                bs.id \
            FROM \
                balances_snapshots bs \
            WHERE \
                bs.id > $1 \
            ORDER BY bs.id ASC \
            LIMIT $2;",
            &[
                &id,
                &LIMIT,
            ],
        )
        .await?;

    if row_registry3.is_empty() {
        return Ok(true);
    }

    let mut insert_query = "INSERT INTO unspentio.balance_snapshot (user_id, asset_id, total_amount, created_at) VALUES".to_string();

    let mut id: i64 = 0;

    'a: for row in row_registry3.iter() {
        let user_id = row.try_get::<'_, usize, Option<i32>>(0)?;

        let asset_id = row.try_get::<'_, usize, Option<String>>(1)?;

        let timestamp = row.try_get::<'_, usize, Option<String>>(2)?;

        let total_amount = row.try_get::<'_, usize, String>(3)?;

        id = row.try_get::<'_, usize, i64>(4)?;

        let user_id_ = match user_id {
            Some(user_id__) => user_id__,
            None => {
                continue 'a;
            }
        };

        let asset_id_ = match asset_id {
            Some(asset_id__) => asset_id__,
            None => {
                continue 'a;
            }
        };

        let timestamp_ = match timestamp {
            Some(timestamp__) => timestamp__,
            None => {
                continue 'a;
            }
        };

        let total_amount_ = total_amount.as_str();

        if total_amount_ == "0" || total_amount_ == "0.0" || total_amount_ == ".0" || total_amount_.contains('-') {
            continue 'a;
        }

        insert_query = format!(
            "{} ({}, \'{}\', toDecimal128(\'{}\', 19), toDateTime(\'{}\', 'UTC')), ",
            insert_query,
            user_id_,
            asset_id_.as_str(),
            total_amount.as_str(),
            timestamp_.as_str()
        );
    }

    clickhouse_client.query(insert_query.as_str()).execute().await?;

    postgresql_client_updater
        .query(
            "INSERT INTO public.balance_snapshot_guard (id, last_inserted_id) VALUES (1, $1) ON CONFLICT (id) DO UPDATE SET last_inserted_id = EXCLUDED.last_inserted_id;",
            &[&id],
        )
        .await?;

    return Ok(false);
}

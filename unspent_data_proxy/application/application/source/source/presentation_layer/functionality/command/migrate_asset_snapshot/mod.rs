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
    let mut offset_factor: i64 = 0;

    'a: loop {
        sleep(Duration::from_secs(1));

        let row_registry1 = postgresql_client_updater
            .query(
                "\
                SELECT \
                    to_char(asg.last_inserted_timestamp, 'YYYY-MM-DD HH24:MI:SS') \
                FROM \
                    public.asset_snapshot_guard asg \
                WHERE \
                    asg.id = 1;",
                &[],
            )
            .await?;

        if row_registry1.is_empty() {
            let row_registry2 = postgresql_client_selector
                .query(
                    "\
                    SELECT \
                        to_char(min(as_.timestamp), 'YYYY-MM-DD HH24:MI:SS') \
                    FROM \
                        public.assets_snapshots as_;",
                    &[],
                )
                .await?;

            if row_registry2.is_empty() {
                return Ok(());
            }

            let lowest_timestamp = row_registry2[0].try_get::<'_, usize, String>(0)?;

            match change_state(
                clickhouse_client,
                postgresql_client_updater,
                postgresql_client_selector,
                lowest_timestamp.as_str(),
                &mut offset_factor,
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
            let lowest_timestamp = row_registry1[0].try_get::<'_, usize, String>(0)?;

            match change_state(
                clickhouse_client,
                postgresql_client_updater,
                postgresql_client_selector,
                lowest_timestamp.as_str(),
                &mut offset_factor,
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
    lowest_timestamp: &'a str,
    offset_factor: &'a mut i64,
) -> Result<bool, Box<dyn Error + 'static>> {
    let offset = match offset_factor.checked_mul(LIMIT) {
        Some(offset_) => offset_,
        None => {
            return Err("There is too much data in the database with the same index.".into());
        }
    };

    println!(
        "Data request with \'timestamp\' = {} and \'offset\' = {} in progress.",
        lowest_timestamp, offset,
    );

    let row_registry3 = postgresql_client_selector
        .query(
            "\
            SELECT \
                as_.asset_id, \
                CASE \
                    WHEN as_.price_usd IS NOT NULL \
                    THEN as_.price_usd::TEXT \
                    ELSE NULL \
                END, \
                CASE \
                    WHEN as_.price_btc IS NOT NULL \
                    THEN as_.price_btc::TEXT \
                    ELSE NULL \
                END, \
                to_char(as_.timestamp, 'YYYY-MM-DD HH24:MI:SS') \
            FROM \
                assets_snapshots as_ \
            WHERE \
                as_.timestamp > (TO_TIMESTAMP($1, 'YYYY-MM-DD HH24:MI:SS') - INTERVAL '1 SECONDS') \
            ORDER BY as_.timestamp ASC \
            LIMIT $2 OFFSET $3;",
            &[
                &lowest_timestamp,
                &LIMIT,
                &offset,
            ],
        )
        .await?;

    if row_registry3.is_empty() {
        return Ok(true);
    }

    let mut insert_query = "INSERT INTO unspentio.asset_snapshot (asset_id, price_usd, price_btc, created_at) VALUES".to_string();

    let mut timestamp = String::new();

    'a: for row in row_registry3.iter() {
        let asset_id = row.try_get::<'_, usize, String>(0)?;

        let price_usd = row.try_get::<'_, usize, Option<String>>(1)?;

        let price_btc = row.try_get::<'_, usize, Option<String>>(2)?;

        timestamp = row.try_get::<'_, usize, String>(3)?;

        let price_usd_ = match price_usd {
            Some(price_usd__) => price_usd__,
            None => {
                continue 'a;
            }
        };

        let price_usd__ = price_usd_.as_str();

        if price_usd__ == "0" || price_usd__ == "0.0" || price_usd__ == ".0" || price_usd__.contains('-') {
            continue 'a;
        }

        match price_btc {
            Some(price_btc_) => {
                let price_btc__ = price_btc_.as_str();

                if price_btc__ == "0" || price_btc__ == "0.0" || price_btc__ == ".0" || price_btc__.contains('-') {
                    continue 'a;
                }

                insert_query = format!(
                    "{} (\'{}\', toDecimal128(\'{}\', 19), toDecimal128(\'{}\', 19),  toDateTime(\'{}\', 'UTC')), ",
                    insert_query,
                    asset_id.as_str(),
                    price_usd_.as_str(),
                    price_btc_.as_str(),
                    timestamp.as_str(),
                );
            }
            None => {
                insert_query = format!(
                    "{} (\'{}\', toDecimal128(\'{}\', 19), NULL,  toDateTime(\'{}\', 'UTC')), ",
                    insert_query,
                    asset_id.as_str(),
                    price_usd_.as_str(),
                    timestamp.as_str(),
                );
            }
        }
    }

    clickhouse_client.query(insert_query.as_str()).execute().await?;

    postgresql_client_updater
        .query(
            "INSERT INTO public.asset_snapshot_guard (id, last_inserted_timestamp) VALUES (1, TO_TIMESTAMP($1, 'YYYY-MM-DD HH24:MI:SS')) ON CONFLICT (id) DO UPDATE SET last_inserted_timestamp = EXCLUDED.last_inserted_timestamp;",
            &[&timestamp],
        )
        .await?;

    if lowest_timestamp == timestamp.as_str() {
        *offset_factor += 1;
    } else {
        *offset_factor = 0;
    }

    return Ok(false);
}

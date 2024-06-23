use super::PostgresqlRepository;
use crate::{
    domain_layer::data::entity::asset::Asset,
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
use tokio_postgres::{
    types::{
        ToSql,
        Type,
    },
    Client,
};

impl PostgresqlRepository<Asset> {
    pub async fn batch_upsert<'a>(client: &'a Client, asset_registry: &'a [Asset]) -> Result<(), Auditor<Error>> {
        if asset_registry.is_empty() {
            return Ok(());
        }

        let mut query = "\
            INSERT INTO public.assets \
            (\
                id, \
                name, \
                symbol, \
                price_usd, \
                price_btc, \
                market_cap_usd, \
                percent_change_24h, \
                percent_change_7d, \
                percent_change_30d, \
                percent_change_1y, \
                percent_change_24h_btc, \
                type, \
                rank, \
                total_supply, \
                circulating_supply, \
                platform_id, \
                last_updated_timestamp, \
                image_url\
            ) \
            VALUES"
            .to_string();

        let maximum_index = asset_registry.len() - 1;

        let mut type_registry: Vec<Type> = vec![];

        let mut value_registry: Vec<Box<dyn ToSql + Sync + Send>> = vec![];

        let mut counter: usize = 0;

        '_a: for (index, asset) in asset_registry.iter().enumerate() {
            counter += 1;

            let counter_1 = counter;

            counter += 1;

            let counter_2 = counter;

            counter += 1;

            let counter_3 = counter;

            counter += 1;

            let counter_4 = counter;

            counter += 1;

            let counter_5 = counter;

            counter += 1;

            let counter_6 = counter;

            counter += 1;

            let counter_7 = counter;

            counter += 1;

            let counter_8 = counter;

            counter += 1;

            let counter_9 = counter;

            counter += 1;

            let counter_10 = counter;

            counter += 1;

            let counter_11 = counter;

            counter += 1;

            let counter_12 = counter;

            counter += 1;

            let counter_13 = counter;

            counter += 1;

            let counter_14 = counter;

            counter += 1;

            let counter_15 = counter;

            counter += 1;

            let counter_16 = counter;

            counter += 1;

            let counter_17 = counter;

            counter += 1;

            let counter_18 = counter;

            query = format!(
                "\
                {} \
                (\
                    ${}, \
                    ${}, \
                    ${}, \
                    CAST(${} AS DECIMAL), \
                    CAST(${} AS DECIMAL), \
                    CAST(${} AS DECIMAL), \
                    CAST(${} AS DECIMAL), \
                    CAST(${} AS DECIMAL), \
                    CAST(${} AS DECIMAL), \
                    CAST(${} AS DECIMAL), \
                    CAST(${} AS DECIMAL), \
                    CAST(${} AS asset_type), \
                    CAST(${} AS DECIMAL), \
                    CAST(${} AS DECIMAL), \
                    CAST(${} AS DECIMAL), \
                    ${}, \
                    TO_TIMESTAMP(${}), \
                    ${}\
                )",
                query.as_str(),
                counter_1,
                counter_2,
                counter_3,
                counter_4,
                counter_5,
                counter_6,
                counter_7,
                counter_8,
                counter_9,
                counter_10,
                counter_11,
                counter_12,
                counter_13,
                counter_14,
                counter_15,
                counter_16,
                counter_17,
                counter_18,
            );

            if index < maximum_index {
                query = format!(
                    "{},",
                    query.as_str()
                );
            }

            let asset_id = asset.id.0.as_str();

            let asset_name = asset.name.as_str();

            let asset_symbol = asset.symbol.as_str();

            let asset_price_usd = match asset.price_usd {
                Some(ref asset_price_usd_) => Some(asset_price_usd_.as_str()),
                None => None,
            };

            let asset_price_btc = match asset.price_btc {
                Some(ref asset_price_btc_) => Some(asset_price_btc_.as_str()),
                None => None,
            };

            let asset_market_cap_usd = match asset.market_cap_usd {
                Some(ref asset_market_cap_usd_) => Some(asset_market_cap_usd_.as_str()),
                None => None,
            };

            let asset_percent_change_24h = match asset.percent_change_24h {
                Some(ref asset_percent_change_24h_) => Some(asset_percent_change_24h_.as_str()),
                None => None,
            };

            let asset_percent_change_7d = match asset.percent_change_7d {
                Some(ref asset_percent_change_7d_) => Some(asset_percent_change_7d_.as_str()),
                None => None,
            };

            let asset_percent_change_30d = match asset.percent_change_30d {
                Some(ref asset_percent_change_30d_) => Some(asset_percent_change_30d_.as_str()),
                None => None,
            };

            let asset_percent_change_1y = match asset.percent_change_1y {
                Some(ref asset_percent_change_1y_) => Some(asset_percent_change_1y_.as_str()),
                None => None,
            };

            let asset_percent_change_24h_btc = match asset.percent_change_24h_btc {
                Some(ref asset_percent_change_24h_btc_) => Some(asset_percent_change_24h_btc_.as_str()),
                None => None,
            };

            let asset_type = asset.r#type.to_string();

            let asset_rank = match asset.rank {
                Some(ref asset_rank_) => Some(asset_rank_.as_str()),
                None => None,
            };

            let asset_total_supply = match asset.total_supply {
                Some(ref asset_total_supply_) => Some(asset_total_supply_.as_str()),
                None => None,
            };

            let asset_circulating_supply = match asset.circulating_supply {
                Some(ref asset_circulating_supply_) => Some(asset_circulating_supply_.as_str()),
                None => None,
            };

            let asset_platform_id = match asset.platform_id {
                Some(ref platform_id_) => Some(platform_id_.as_str()),
                None => None,
            };

            let asset_image_url = match asset.image_url {
                Some(ref image_url_) => Some(image_url_.as_str()),
                None => None,
            };

            value_registry.push(Box::new(asset_id));

            value_registry.push(Box::new(asset_name));

            value_registry.push(Box::new(asset_symbol));

            value_registry.push(Box::new(asset_price_usd));

            value_registry.push(Box::new(asset_price_btc));

            value_registry.push(Box::new(asset_market_cap_usd));

            value_registry.push(Box::new(asset_percent_change_24h));

            value_registry.push(Box::new(asset_percent_change_7d));

            value_registry.push(Box::new(asset_percent_change_30d));

            value_registry.push(Box::new(asset_percent_change_1y));

            value_registry.push(Box::new(asset_percent_change_24h_btc));

            value_registry.push(Box::new(asset_type));

            value_registry.push(Box::new(asset_rank));

            value_registry.push(Box::new(asset_total_supply));

            value_registry.push(Box::new(asset_circulating_supply));

            value_registry.push(Box::new(asset_platform_id));

            value_registry.push(Box::new(asset.last_updated_timestamp));

            value_registry.push(Box::new(asset_image_url));

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::TEXT);

            type_registry.push(Type::INT8);

            type_registry.push(Type::TEXT);
        }

        let mut value_registry_: Vec<&(dyn ToSql + Sync)> = vec![];

        '_a: for value in value_registry.iter() {
            value_registry_.push(value.as_ref());
        }

        query = format!(
            "\
            {} \
            ON CONFLICT ON CONSTRAINT assets_pkey \
            DO UPDATE \
            SET \
            (\
                name, \
                symbol, \
                price_usd, \
                price_btc, \
                market_cap_usd, \
                percent_change_24h, \
                percent_change_7d, \
                percent_change_30d, \
                percent_change_1y, \
                percent_change_24h_btc, \
                rank, \
                total_supply, \
                circulating_supply, \
                last_updated_timestamp, \
                image_url\
            ) = (\
                EXCLUDED.name, \
                EXCLUDED.symbol, \
                EXCLUDED.price_usd, \
                EXCLUDED.price_btc, \
                EXCLUDED.market_cap_usd, \
                EXCLUDED.percent_change_24h, \
                EXCLUDED.percent_change_7d, \
                EXCLUDED.percent_change_30d, \
                EXCLUDED.percent_change_1y, \
                EXCLUDED.percent_change_24h_btc, \
                EXCLUDED.rank, \
                EXCLUDED.total_supply, \
                EXCLUDED.circulating_supply, \
                EXCLUDED.last_updated_timestamp, \
                EXCLUDED.image_url\
            );",
            query.as_str(),
        );

        let statement = match client
            .prepare_typed(
                query.as_str(),
                type_registry.as_slice(),
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
                value_registry_.as_slice(),
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

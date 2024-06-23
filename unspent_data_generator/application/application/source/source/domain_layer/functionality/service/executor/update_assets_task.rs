use super::Executor;
use crate::{
    application_layer::data::unified_report::UnifiedReport,
    domain_layer::data::entity::{
        asset::{
            Asset,
            Asset_Id,
            Asset_Type,
        },
        task::{
            update_assets::UpdateAssets,
            Task,
        },
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            control_type::{
                Json,
                UTCDateTime,
            },
            environment_configuration::EnvironmentConfiguration,
            error::{
                Error,
                Other,
                Runtime,
            },
            void::Void,
        },
        functionality::{
            repository::postgresql::PostgresqlRepository,
            service::{
                creator::Creator,
                formatter::Formatter,
                resolver::Resolver,
                serializer::{
                    Serialize as _,
                    Serializer,
                },
                spawner::{
                    tokio_non_blocking_task::TokioNonBlockingTask,
                    Spawner,
                },
            },
        },
        macro_rules::coingecko_response_status_pattern,
    },
};
use bytes::Buf;
use http::{
    header::{
        ACCEPT,
        CONTENT_TYPE,
    },
    HeaderValue,
};
use reqwest::{
    Body,
    Client as ReqwestClient,
};
use rust_decimal::{
    Decimal,
    RoundingStrategy,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::{
    sync::Mutex,
    task::JoinHandle,
    time::{
        sleep,
        Duration,
    },
};
use tokio_postgres::Client;

static QUANTITY_OF_CONSECUTIVE_TERMINATIONS_WITH_ERRORS: Mutex<usize> = Mutex::const_new(0);

impl Executor<Task<UpdateAssets>> {
    // Coingecko API limitation. Can be less or equal to 250.
    const MAXIMUM_ASSETS_QUANTITY_IN_QUERY: u8 = 250;

    const PRO_API_KEY_HEADER_KEY: &'static str = "x-cg-pro-api-key";

    const ACCEPT_HEADER_VALUE: &'static str = "application/json";

    const TASK_REPETITIONS_QUANTITY_1: u8 = 2;

    const TASK_REPETITIONS_QUANTITY_2: u8 = 3;

    const TASK_REPETITIONS_QUANTITY_3: u8 = 10;

    const TASK_SLEEPING_TIME_AFTER_ERROR_IN_SECONDS_1: u64 = 10;

    const TASK_SLEEPING_TIME_AFTER_ERROR_IN_SECONDS_2: u64 = 5;

    const TASK_SLEEPING_TIME_AFTER_ERROR_IN_SECONDS_3: u64 = 1;

    pub async fn get_quantity_of_consecutive_terminations_with_errors() -> usize {
        return *(QUANTITY_OF_CONSECUTIVE_TERMINATIONS_WITH_ERRORS.lock().await);
    }

    pub async fn execute(environment_configuration: Arc<EnvironmentConfiguration>) -> Result<(), Auditor<Error>> {
        let mut counter: u8 = 0;

        'a: loop {
            if let Err(mut error_auditor) = Self::execute_(environment_configuration.clone()).await {
                if counter < Self::TASK_REPETITIONS_QUANTITY_1 {
                    counter += 1;

                    sleep(Duration::from_secs(Self::TASK_SLEEPING_TIME_AFTER_ERROR_IN_SECONDS_1)).await;

                    continue 'a;
                } else {
                    error_auditor.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    );

                    return Err(error_auditor);
                }
            }

            break 'a;
        }

        {
            *(QUANTITY_OF_CONSECUTIVE_TERMINATIONS_WITH_ERRORS.lock().await) = 0;
        }

        return Ok(());
    }

    async fn execute_(environment_configuration: Arc<EnvironmentConfiguration>) -> Result<(), Auditor<Error>> {
        let asset_last_updated_timestamp = Resolver::<UTCDateTime>::get_now();

        let mut join_handle_registry: Vec<JoinHandle<Result<(), Auditor<Error>>>> = vec![];

        let mut environment_configuration_ = environment_configuration.clone();

        let update_crypto_asset_future = async move {
            let mut result = Ok(());

            if let Err(mut error_auditor) = Self::update_crypto_asset(
                environment_configuration_,
                asset_last_updated_timestamp,
            )
            .await
            {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                result = Err(error_auditor);
            }

            result
        };

        environment_configuration_ = environment_configuration.clone();

        let update_fiat_asset_future = async move {
            let mut result = Ok(());

            if let Err(mut error_auditor) = Self::update_fiat_asset(
                environment_configuration_,
                asset_last_updated_timestamp,
            )
            .await
            {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                result = Err(error_auditor);
            }

            result
        };

        join_handle_registry.push(Spawner::<TokioNonBlockingTask>::spawn_processed(update_crypto_asset_future));

        join_handle_registry.push(Spawner::<TokioNonBlockingTask>::spawn_processed(update_fiat_asset_future));

        let mut error_auditor_registry: Vec<Auditor<Error>> = vec![];

        '_a: for join_handle in join_handle_registry.into_iter() {
            let result = match join_handle.await {
                Ok(result_) => result_,
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

            if let Err(mut error_auditor) = result {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                error_auditor_registry.push(error_auditor);
            }
        }

        if !error_auditor_registry.is_empty() {
            let error_message = Formatter::<&'_ [Auditor<Error>]>::format(error_auditor_registry.as_slice());

            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new_(error_message.into()),
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

    async fn update_crypto_asset(environment_configuration: Arc<EnvironmentConfiguration>, asset_last_updated_timestamp: i64) -> Result<(), Auditor<Error>> {
        let mut counter: u8 = 0;

        'a: loop {
            if let Err(mut error_auditor) = Self::update_crypto_asset_(
                environment_configuration.clone(),
                asset_last_updated_timestamp,
            )
            .await
            {
                if counter < Self::TASK_REPETITIONS_QUANTITY_2 {
                    counter += 1;

                    sleep(Duration::from_secs(Self::TASK_SLEEPING_TIME_AFTER_ERROR_IN_SECONDS_2)).await;

                    continue 'a;
                } else {
                    error_auditor.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    );

                    return Err(error_auditor);
                }
            }

            break 'a;
        }

        return Ok(());
    }

    async fn update_crypto_asset_(environment_configuration: Arc<EnvironmentConfiguration>, asset_last_updated_timestamp: i64) -> Result<(), Auditor<Error>> {
        let crypto_asset_registry = match Self::request_crypto_asset_registry(environment_configuration.clone()).await {
            Ok(asset_registry_) => asset_registry_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        let usd_btc_current_price = match Self::request_usd_btc_current_price(environment_configuration.clone()).await {
            Ok(usd_btc_current_price_) => usd_btc_current_price_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        let usd_btc_current_price_ = match Decimal::from_f64_retain(usd_btc_current_price) {
            Some(usd_btc_current_price__) => usd_btc_current_price__,
            None => {
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
        };

        let crypto_asset_registry_registry = Self::split_asset_registry(
            crypto_asset_registry,
            Self::MAXIMUM_ASSETS_QUANTITY_IN_QUERY as usize,
        );

        let mut join_handle_registry: Vec<JoinHandle<Result<(), Auditor<Error>>>> = vec![];

        '_a: for crypto_asset_registry_ in crypto_asset_registry_registry.into_iter() {
            sleep(Duration::from_millis(200)).await;

            let environment_configuration_ = environment_configuration.clone();

            let future = async move {
                let mut result = Ok(());

                if let Err(mut error_auditor) = Self::update_crypto_asset_part(
                    environment_configuration_,
                    crypto_asset_registry_,
                    usd_btc_current_price_,
                    asset_last_updated_timestamp,
                )
                .await
                {
                    error_auditor.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    );

                    result = Err(error_auditor);
                }

                result
            };

            join_handle_registry.push(Spawner::<TokioNonBlockingTask>::spawn_processed(future));
        }

        let mut error_auditor_registry: Vec<Auditor<Error>> = vec![];

        '_a: for join_handle in join_handle_registry.into_iter() {
            let result = match join_handle.await {
                Ok(result_) => result_,
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

            if let Err(mut error_auditor) = result {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                error_auditor_registry.push(error_auditor);
            }
        }

        if !error_auditor_registry.is_empty() {
            let error_message = Formatter::<&'_ [Auditor<Error>]>::format(error_auditor_registry.as_slice());

            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new_(error_message.into()),
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

    async fn request_crypto_asset_registry(environment_configuration: Arc<EnvironmentConfiguration>) -> Result<Vec<CryptoAsset_>, Auditor<Error>> {
        let environment_configuration_ = environment_configuration.as_ref();

        let url = format!(
            "{}/api/v3/coins/list?include_platform=false&status=active",
            environment_configuration_.remote_service.coingecko.pro.url.as_str()
        );

        let mut request_builder = match ReqwestClient::builder().build() {
            Ok(client_) => client_.get(url.as_str()),
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

        let header_value = match HeaderValue::from_str(environment_configuration_.remote_service.coingecko.pro.api_key.as_str()) {
            Ok(header_value_) => header_value_,
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

        request_builder = request_builder
            .header(
                ACCEPT,
                HeaderValue::from_static(Self::ACCEPT_HEADER_VALUE),
            )
            .header(
                Self::PRO_API_KEY_HEADER_KEY,
                header_value,
            );

        let response = match request_builder.send().await {
            Ok(response_) => response_,
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

        if response.status().is_client_error() || response.status().is_server_error() {
            let error_message = format!(
                coingecko_response_status_pattern!(),
                response.status().as_str(),
            );

            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new_(error_message.into()),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        let bytes = match response.bytes().await {
            Ok(bytes_) => bytes_,
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

        let crypto_asset_registry = match Serializer::<Json>::deserialize::<'_, Vec<CryptoAsset_>>(bytes.chunk()) {
            Ok(crypto_asset_registry_) => crypto_asset_registry_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        return Ok(crypto_asset_registry);
    }

    fn split_asset_registry(crypto_asset_registry: Vec<CryptoAsset_>, elements_quantity: usize) -> Vec<Vec<CryptoAsset_>> {
        let mut crypto_asset_registry_registry: Vec<Vec<CryptoAsset_>> = vec![];

        let mut counter: usize = 0;

        let mut crypto_asset_registry_: Vec<CryptoAsset_> = vec![];

        for crypto_asset in crypto_asset_registry.into_iter() {
            counter += 1;

            crypto_asset_registry_.push(crypto_asset);

            if counter == elements_quantity {
                counter = 0;

                crypto_asset_registry_registry.push(crypto_asset_registry_);

                crypto_asset_registry_ = vec![];
            }
        }

        if !crypto_asset_registry_.is_empty() {
            crypto_asset_registry_registry.push(crypto_asset_registry_);
        }

        return crypto_asset_registry_registry;
    }

    async fn request_usd_btc_current_price<'a>(environment_configuration: Arc<EnvironmentConfiguration>) -> Result<f64, Auditor<Error>> {
        let usd_detailed_crypto_asset_btc = match Self::request_usd_detailed_crypto_asset_btc(environment_configuration.clone()).await {
            Ok(usd_detailed_crypto_asset_btc_) => usd_detailed_crypto_asset_btc_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        let btc_usd_current_price = match usd_detailed_crypto_asset_btc.current_price {
            Some(btc_usd_current_price_) => btc_usd_current_price_,
            None => {
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
        };

        let usd_btc_current_price = (1.0 as f64) / btc_usd_current_price;

        if !usd_btc_current_price.is_finite() || usd_btc_current_price.is_subnormal() {
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

        return Ok(usd_btc_current_price);
    }

    async fn update_crypto_asset_part<'a>(
        environment_configuration: Arc<EnvironmentConfiguration>,
        crypto_asset_registry: Vec<CryptoAsset_>,
        usd_btc_current_price: Decimal,
        asset_last_updated_timestamp: i64,
    ) -> Result<(), Auditor<Error>> {
        let mut counter: u8 = 0;

        'a: loop {
            if let Err(mut error_auditor) = Self::update_crypto_asset_part_(
                environment_configuration.clone(),
                crypto_asset_registry.as_slice(),
                usd_btc_current_price,
                asset_last_updated_timestamp,
            )
            .await
            {
                if counter < Self::TASK_REPETITIONS_QUANTITY_3 {
                    counter += 1;

                    sleep(Duration::from_secs(Self::TASK_SLEEPING_TIME_AFTER_ERROR_IN_SECONDS_3)).await;

                    continue 'a;
                } else {
                    error_auditor.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    );

                    return Err(error_auditor);
                }
            }

            break 'a;
        }

        return Ok(());
    }

    async fn update_crypto_asset_part_<'a>(
        environment_configuration: Arc<EnvironmentConfiguration>,
        crypto_asset_registry: &'a [CryptoAsset_],
        usd_btc_current_price: Decimal,
        asset_last_updated_timestamp: i64,
    ) -> Result<(), Auditor<Error>> {
        struct DetailedCryptoAsset_ {
            current_price: Option<f64>,
            price_change_percentage_24h: Option<f64>,
        }

        let mut ids_parameter = String::new();

        '_a: for (index, crypto_asset) in crypto_asset_registry.iter().enumerate() {
            if index == 0 {
                ids_parameter = format!(
                    "{}{}",
                    ids_parameter.as_str(),
                    crypto_asset.id.as_str()
                );
            } else {
                ids_parameter = format!(
                    "{},{}",
                    ids_parameter.as_str(),
                    crypto_asset.id.as_str()
                );
            }
        }

        let usd_detailed_crypto_asset_registry = match Self::request_usd_detailed_crypto_asset_registry(
            environment_configuration.clone(),
            ids_parameter.as_str(),
        )
        .await
        {
            Ok(usd_detailed_crypto_asset_registry_) => usd_detailed_crypto_asset_registry_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        let mut asset_id_registry: Vec<&'_ str> = vec![];

        '_a: for crypto_asset in crypto_asset_registry.iter() {
            asset_id_registry.push(crypto_asset.id.as_str());
        }

        let asset_snapshot_for_price_difference_percentage_calculating_registry = match Self::request_asset_registry_for_price_difference_percentage_calculating(
            environment_configuration.clone(),
            asset_id_registry.as_slice(),
        )
        .await
        {
            Ok(asset_snapshot_for_price_difference_percentage_calculating_registry_) => asset_snapshot_for_price_difference_percentage_calculating_registry_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        let mut asset_snapshot_for_price_difference_percentage_calculating_hash_map = HashMap::<String, AssetSnapshotForPriceDifferencePercentageCalculating_>::new();

        '_a: for asset_snapshot_for_price_difference_percentage_calculating in asset_snapshot_for_price_difference_percentage_calculating_registry.into_iter() {
            asset_snapshot_for_price_difference_percentage_calculating_hash_map.insert(
                asset_snapshot_for_price_difference_percentage_calculating.asset_id,
                AssetSnapshotForPriceDifferencePercentageCalculating_ {
                    price_usd_24_hours: asset_snapshot_for_price_difference_percentage_calculating.price_usd_24_hours,
                    price_btc_24_hours: asset_snapshot_for_price_difference_percentage_calculating.price_btc_24_hours,
                    price_usd_7_days: asset_snapshot_for_price_difference_percentage_calculating.price_usd_7_days,
                    price_usd_30_days: asset_snapshot_for_price_difference_percentage_calculating.price_usd_30_days,
                    price_usd_1_year: asset_snapshot_for_price_difference_percentage_calculating.price_usd_1_year,
                },
            );
        }

        let mut asset_registry_: Vec<Asset> = vec![];

        'a: for usd_detailed_crypto_asset in usd_detailed_crypto_asset_registry.into_iter() {
            let asset_id = Asset_Id(usd_detailed_crypto_asset.id);

            let asset_rank = match usd_detailed_crypto_asset.market_cap_rank {
                Some(market_cap_rank) => Some(market_cap_rank.to_string()),
                None => None,
            };

            let (asset_price_usd, asset_price_btc, asset_percent_change_7d, asset_percent_change_30d, asset_percent_change_1y, asset_percent_change_24h_btc) =
                match usd_detailed_crypto_asset.current_price {
                    Some(current_price) => {
                        let current_price_ = match Decimal::from_f64_retain(current_price) {
                            Some(current_price__) => current_price__,
                            None => {
                                continue 'a;
                            }
                        };

                        let asset_price_btc_ = current_price_ * usd_btc_current_price;

                        let (asset_percent_change_7d_, asset_percent_change_30d_, asset_percent_change_1y_, asset_percent_change_24h_btc_) =
                            match asset_snapshot_for_price_difference_percentage_calculating_hash_map.remove(asset_id.0.as_str()) {
                                Some(asset_snapshot_for_price_difference_percentage_calculating) => {
                                    (
                                        Self::get_transformed_calculated_price_difference_percentage(
                                            asset_snapshot_for_price_difference_percentage_calculating.price_usd_7_days,
                                            current_price_,
                                        ),
                                        Self::get_transformed_calculated_price_difference_percentage(
                                            asset_snapshot_for_price_difference_percentage_calculating.price_usd_30_days,
                                            current_price_,
                                        ),
                                        Self::get_transformed_calculated_price_difference_percentage(
                                            asset_snapshot_for_price_difference_percentage_calculating.price_usd_1_year,
                                            current_price_,
                                        ),
                                        Self::get_transformed_calculated_price_difference_percentage(
                                            asset_snapshot_for_price_difference_percentage_calculating.price_btc_24_hours,
                                            asset_price_btc_,
                                        ),
                                    )
                                }
                                None => {
                                    (
                                        None,
                                        None,
                                        None,
                                        None,
                                    )
                                }
                            };

                        (
                            Some(current_price_.to_string()),
                            Some(asset_price_btc_.to_string()),
                            asset_percent_change_7d_,
                            asset_percent_change_30d_,
                            asset_percent_change_1y_,
                            asset_percent_change_24h_btc_,
                        )
                    }
                    None => {
                        continue 'a;
                    }
                };

            let asset_market_cap_usd = match usd_detailed_crypto_asset.market_cap {
                Some(market_cap) => Some(market_cap.to_string()),
                None => None,
            };

            let asset_percent_change_24h = match usd_detailed_crypto_asset.price_change_percentage_24h {
                Some(price_change_percentage_24h) => Some(price_change_percentage_24h.to_string()),
                None => None,
            };

            let asset_total_supply = match usd_detailed_crypto_asset.total_supply {
                Some(total_supply) => Some(total_supply.to_string()),
                None => None,
            };

            let asset_circulating_supply = match usd_detailed_crypto_asset.circulating_supply {
                Some(circulating_supply) => Some(circulating_supply.to_string()),
                None => None,
            };

            asset_registry_.push(
                Asset::new(
                    asset_id,
                    usd_detailed_crypto_asset.name,
                    usd_detailed_crypto_asset.symbol.to_uppercase(),
                    asset_price_usd,
                    asset_price_btc,
                    asset_market_cap_usd,
                    asset_percent_change_24h,
                    asset_percent_change_7d,
                    asset_percent_change_30d,
                    asset_percent_change_1y,
                    asset_percent_change_24h_btc,
                    Asset_Type::Crypto,
                    asset_rank,
                    asset_total_supply,
                    asset_circulating_supply,
                    None,
                    asset_last_updated_timestamp,
                    usd_detailed_crypto_asset.image,
                ),
            );
        }

        let client = match Creator::<Client>::create(environment_configuration.as_ref()).await {
            Ok(client_) => client_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        if let Err(mut error_auditor) = PostgresqlRepository::<Asset>::batch_upsert(
            &client,
            asset_registry_.as_slice(),
        )
        .await
        {
            error_auditor.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                ),
            );

            return Err(error_auditor);
        }

        return Ok(());
    }

    async fn request_usd_detailed_crypto_asset_btc<'a>(environment_configuration: Arc<EnvironmentConfiguration>) -> Result<DetailedCryptoAsset, Auditor<Error>> {
        let url = format!(
            "{}/api/v3/coins/markets?vs_currency=usd&locale=en&page=1&per_page=1&ids=bitcoin",
            environment_configuration.as_ref().remote_service.coingecko.pro.url.as_str(),
        );

        let mut detailed_crypto_asset_registry = match Self::request_detailed_crypto_asset_registry(
            url.as_str(),
            environment_configuration,
        )
        .await
        {
            Ok(detailed_crypto_asset_registry_) => detailed_crypto_asset_registry_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        let detailed_crypto_asset = match detailed_crypto_asset_registry.pop() {
            Some(detailed_crypto_asset_) => detailed_crypto_asset_,
            None => {
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
        };

        return Ok(detailed_crypto_asset);
    }

    async fn request_usd_detailed_crypto_asset_registry<'a>(
        environment_configuration: Arc<EnvironmentConfiguration>,
        ids_parameter: &'a str,
    ) -> Result<Vec<DetailedCryptoAsset>, Auditor<Error>> {
        let url = format!(
            "{}/api/v3/coins/markets?vs_currency=usd&locale=en&page=1&per_page={}&ids={}",
            environment_configuration.as_ref().remote_service.coingecko.pro.url.as_str(),
            Self::MAXIMUM_ASSETS_QUANTITY_IN_QUERY,
            ids_parameter,
        );

        let detailed_crypto_asset_registry = match Self::request_detailed_crypto_asset_registry(
            url.as_str(),
            environment_configuration,
        )
        .await
        {
            Ok(detailed_crypto_asset_registry_) => detailed_crypto_asset_registry_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        return Ok(detailed_crypto_asset_registry);
    }

    async fn request_detailed_crypto_asset_registry<'a>(
        url: &'a str,
        environment_configuration: Arc<EnvironmentConfiguration>,
    ) -> Result<Vec<DetailedCryptoAsset>, Auditor<Error>> {
        let mut request_builder = match ReqwestClient::builder().build() {
            Ok(client_) => client_.get(url),
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

        let header_value = match HeaderValue::from_str(environment_configuration.as_ref().remote_service.coingecko.pro.api_key.as_str()) {
            Ok(header_value_) => header_value_,
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

        request_builder = request_builder
            .header(
                ACCEPT,
                HeaderValue::from_static(Self::ACCEPT_HEADER_VALUE),
            )
            .header(
                Self::PRO_API_KEY_HEADER_KEY,
                header_value,
            );

        let response = match request_builder.send().await {
            Ok(response_) => response_,
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

        if response.status().is_client_error() || response.status().is_server_error() {
            let error_message = format!(
                coingecko_response_status_pattern!(),
                response.status().as_str(),
            );

            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new_(error_message.into()),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        let bytes = match response.bytes().await {
            Ok(bytes_) => bytes_,
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

        let detailed_crypto_asset_registry = match Serializer::<Json>::deserialize::<'_, Vec<DetailedCryptoAsset>>(bytes.chunk()) {
            Ok(detailed_crypto_asset_registry_) => detailed_crypto_asset_registry_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        return Ok(detailed_crypto_asset_registry);
    }

    async fn request_asset_registry_for_price_difference_percentage_calculating<'a>(
        environment_configuration: Arc<EnvironmentConfiguration>,
        asset_id_registry: &'a [&'_ str],
    ) -> Result<Vec<AssetSnapshotForPriceDifferencePercentageCalculating>, Auditor<Error>> {
        let mut counter: u8 = 0;

        let asset_snapshot_for_price_difference_percentage_calculating_registry = 'a: loop {
            let asset_snapshot_for_price_difference_percentage_calculating_registry_ = match Self::request_asset_registry_for_price_difference_percentage_calculating_(
                environment_configuration.clone(),
                asset_id_registry,
            )
            .await
            {
                Ok(asset_snapshot_for_price_difference_percentage_calculating_registry__) => asset_snapshot_for_price_difference_percentage_calculating_registry__,
                Err(mut error_auditor) => {
                    if counter < Self::TASK_REPETITIONS_QUANTITY_3 {
                        counter += 1;

                        sleep(Duration::from_secs(Self::TASK_SLEEPING_TIME_AFTER_ERROR_IN_SECONDS_3)).await;

                        continue 'a;
                    } else {
                        error_auditor.add_backtrace_part(
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        );

                        return Err(error_auditor);
                    }
                }
            };

            break 'a asset_snapshot_for_price_difference_percentage_calculating_registry_;
        };

        return Ok(asset_snapshot_for_price_difference_percentage_calculating_registry);
    }

    async fn request_asset_registry_for_price_difference_percentage_calculating_<'a>(
        environment_configuration: Arc<EnvironmentConfiguration>,
        asset_id_registry: &'a [&'_ str],
    ) -> Result<Vec<AssetSnapshotForPriceDifferencePercentageCalculating>, Auditor<Error>> {
        #[derive(Serialize)]
        struct RequestBodyData<'a, 'b> {
            server_access_token: &'a str,
            asset_id_registry: &'a [&'b str],
        }

        #[derive(Deserialize)]
        pub struct Outcoming {
            asset_snapshot_for_price_difference_percentage_calculating_registry: Vec<AssetSnapshotForPriceDifferencePercentageCalculating>,
        }

        let request_body_data = RequestBodyData {
            server_access_token: environment_configuration.remote_service.data_proxy.server_access_token.as_str(),
            asset_id_registry,
        };

        let request_body_data_ = match Serializer::<Json>::serialize(&request_body_data) {
            Ok(request_body_data_) => request_body_data_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        let request_client = match ReqwestClient::builder().build() {
            Ok(request_client_) => request_client_,
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

        let request_builder = request_client
            .post(
                format!(
                    "{}/asset_snapshot/history_for_price_difference_percentage_calculating",
                    environment_configuration.remote_service.data_proxy.url.as_str(),
                ),
            )
            .header(
                CONTENT_TYPE,
                HeaderValue::from_static("application/octet-stream"),
            )
            .body(Body::from(request_body_data_));

        let response = match request_builder.send().await {
            Ok(response_) => response_,
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

        if !response.status().is_success() {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new_("Response status code is not 200.".into()),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        let response_body_data = match response.bytes().await {
            Ok(response_body_data_) => response_body_data_,
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

        let unified_report = match Serializer::<Json>::deserialize::<UnifiedReport<Outcoming, Void>>(response_body_data.chunk()) {
            Ok(unified_report_) => unified_report_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        let outcoming = match unified_report.get_data() {
            Some(outcoming_) => outcoming_,
            None => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new_("Unexpected response data.".into()),
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

        return Ok(outcoming.asset_snapshot_for_price_difference_percentage_calculating_registry);
    }

    async fn update_fiat_asset<'a>(environment_configuration: Arc<EnvironmentConfiguration>, asset_last_updated_timestamp: i64) -> Result<(), Auditor<Error>> {
        let mut counter: u8 = 0;

        'a: loop {
            if let Err(mut error_auditor) = Self::update_fiat_asset_(
                environment_configuration.clone(),
                asset_last_updated_timestamp,
            )
            .await
            {
                if counter < Self::TASK_REPETITIONS_QUANTITY_2 {
                    counter += 1;

                    sleep(Duration::from_secs(Self::TASK_SLEEPING_TIME_AFTER_ERROR_IN_SECONDS_2)).await;

                    continue 'a;
                } else {
                    error_auditor.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    );

                    return Err(error_auditor);
                }
            }

            break 'a;
        }

        return Ok(());
    }

    async fn update_fiat_asset_(environment_configuration: Arc<EnvironmentConfiguration>, asset_last_updated_timestamp: i64) -> Result<(), Auditor<Error>> {
        let fiat_asset_registry = match Self::request_fiat_asset_registry(environment_configuration.clone()).await {
            Ok(fiat_asset_registry_) => fiat_asset_registry_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        let mut fiat_asset_registry_: Vec<FiatAsset_> = vec![];

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.usd.name,
            "USD",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.aed.value,
            fiat_asset_registry.rates.aed.name,
            "AED",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.ars.value,
            fiat_asset_registry.rates.ars.name,
            "ARS",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.aud.value,
            fiat_asset_registry.rates.aud.name,
            "AUD",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.bdt.value,
            fiat_asset_registry.rates.bdt.name,
            "BDT",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.bhd.value,
            fiat_asset_registry.rates.bhd.name,
            "BHD",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.bmd.value,
            fiat_asset_registry.rates.bmd.name,
            "BMD",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.brl.value,
            fiat_asset_registry.rates.brl.name,
            "BRL",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.cad.value,
            fiat_asset_registry.rates.cad.name,
            "CAD",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.chf.value,
            fiat_asset_registry.rates.chf.name,
            "CHF",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.clp.value,
            fiat_asset_registry.rates.clp.name,
            "CLP",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.cny.value,
            fiat_asset_registry.rates.cny.name,
            "CNY",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.czk.value,
            fiat_asset_registry.rates.czk.name,
            "CZK",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.dkk.value,
            fiat_asset_registry.rates.dkk.name,
            "DKK",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.eur.value,
            fiat_asset_registry.rates.eur.name,
            "EUR",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.gbp.value,
            fiat_asset_registry.rates.gbp.name,
            "GBP",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.gel.value,
            fiat_asset_registry.rates.gel.name,
            "GEL",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.hkd.value,
            fiat_asset_registry.rates.hkd.name,
            "HKD",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.huf.value,
            fiat_asset_registry.rates.huf.name,
            "HUF",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.idr.value,
            fiat_asset_registry.rates.idr.name,
            "IDR",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.ils.value,
            fiat_asset_registry.rates.ils.name,
            "ILS",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.inr.value,
            fiat_asset_registry.rates.inr.name,
            "INR",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.jpy.value,
            fiat_asset_registry.rates.jpy.name,
            "JPY",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.krw.value,
            fiat_asset_registry.rates.krw.name,
            "KRW",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.kwd.value,
            fiat_asset_registry.rates.kwd.name,
            "KWD",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.lkr.value,
            fiat_asset_registry.rates.lkr.name,
            "LKR",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.mmk.value,
            fiat_asset_registry.rates.mmk.name,
            "MMK",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.mxn.value,
            fiat_asset_registry.rates.mxn.name,
            "MXN",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.myr.value,
            fiat_asset_registry.rates.myr.name,
            "MYR",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.ngn.value,
            fiat_asset_registry.rates.ngn.name,
            "NGN",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.nok.value,
            fiat_asset_registry.rates.nok.name,
            "NOK",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.nzd.value,
            fiat_asset_registry.rates.nzd.name,
            "NZD",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.php.value,
            fiat_asset_registry.rates.php.name,
            "PHP",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.pkr.value,
            fiat_asset_registry.rates.pkr.name,
            "PKR",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.pln.value,
            fiat_asset_registry.rates.pln.name,
            "PLN",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.rub.value,
            fiat_asset_registry.rates.rub.name,
            "RUB",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.sar.value,
            fiat_asset_registry.rates.sar.name,
            "SAR",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.sek.value,
            fiat_asset_registry.rates.sek.name,
            "SEK",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.sgd.value,
            fiat_asset_registry.rates.sgd.name,
            "SGD",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.thb.value,
            fiat_asset_registry.rates.thb.name,
            "THB",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.r#try.value,
            fiat_asset_registry.rates.r#try.name,
            "TRY",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.twd.value,
            fiat_asset_registry.rates.twd.name,
            "TWD",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.uah.value,
            fiat_asset_registry.rates.uah.name,
            "UAH",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.vef.value,
            fiat_asset_registry.rates.vef.name,
            "VEF",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.vnd.value,
            fiat_asset_registry.rates.vnd.name,
            "VND",
        );

        Self::checked_add_fiat_asset(
            &mut fiat_asset_registry_,
            fiat_asset_registry.rates.usd.value,
            fiat_asset_registry.rates.zar.value,
            fiat_asset_registry.rates.zar.name,
            "ZAR",
        );

        let mut asset_id_registry: Vec<&'_ str> = vec![];

        '_a: for fiat_asset in fiat_asset_registry_.iter() {
            asset_id_registry.push(fiat_asset.id.as_str());
        }

        let asset_snapshot_for_price_difference_percentage_calculating_registry = match Self::request_asset_registry_for_price_difference_percentage_calculating(
            environment_configuration.clone(),
            asset_id_registry.as_slice(),
        )
        .await
        {
            Ok(asset_snapshot_for_price_difference_percentage_calculating_registry_) => asset_snapshot_for_price_difference_percentage_calculating_registry_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        let mut asset_snapshot_for_price_difference_percentage_calculating_hash_map = HashMap::<String, AssetSnapshotForPriceDifferencePercentageCalculating_>::new();

        '_a: for asset_snapshot_for_price_difference_percentage_calculating in asset_snapshot_for_price_difference_percentage_calculating_registry.into_iter() {
            asset_snapshot_for_price_difference_percentage_calculating_hash_map.insert(
                asset_snapshot_for_price_difference_percentage_calculating.asset_id,
                AssetSnapshotForPriceDifferencePercentageCalculating_ {
                    price_usd_24_hours: asset_snapshot_for_price_difference_percentage_calculating.price_usd_24_hours,
                    price_btc_24_hours: asset_snapshot_for_price_difference_percentage_calculating.price_btc_24_hours,
                    price_usd_7_days: asset_snapshot_for_price_difference_percentage_calculating.price_usd_7_days,
                    price_usd_30_days: asset_snapshot_for_price_difference_percentage_calculating.price_usd_30_days,
                    price_usd_1_year: asset_snapshot_for_price_difference_percentage_calculating.price_usd_1_year,
                },
            );
        }

        let mut asset_registry_: Vec<Asset> = vec![];

        '_a: for fiat_asset in fiat_asset_registry_.into_iter() {
            let (asset_percent_change_24h, asset_percent_change_7d, asset_percent_change_30d, asset_percent_change_1y, asset_percent_change_24h_btc) =
                match asset_snapshot_for_price_difference_percentage_calculating_hash_map.remove(fiat_asset.id.as_str()) {
                    Some(asset_snapshot_for_price_difference_percentage_calculating) => {
                        (
                            Self::get_transformed_calculated_price_difference_percentage(
                                Some(asset_snapshot_for_price_difference_percentage_calculating.price_usd_24_hours),
                                fiat_asset.price_usd,
                            ),
                            Self::get_transformed_calculated_price_difference_percentage(
                                asset_snapshot_for_price_difference_percentage_calculating.price_usd_7_days,
                                fiat_asset.price_usd,
                            ),
                            Self::get_transformed_calculated_price_difference_percentage(
                                asset_snapshot_for_price_difference_percentage_calculating.price_usd_30_days,
                                fiat_asset.price_usd,
                            ),
                            Self::get_transformed_calculated_price_difference_percentage(
                                asset_snapshot_for_price_difference_percentage_calculating.price_usd_1_year,
                                fiat_asset.price_usd,
                            ),
                            Self::get_transformed_calculated_price_difference_percentage(
                                asset_snapshot_for_price_difference_percentage_calculating.price_btc_24_hours,
                                fiat_asset.price_btc,
                            ),
                        )
                    }
                    None => {
                        (
                            None,
                            None,
                            None,
                            None,
                            None,
                        )
                    }
                };

            asset_registry_.push(
                Asset::new(
                    Asset_Id(fiat_asset.id),
                    fiat_asset.name,
                    fiat_asset.symbol.to_string(),
                    Some(fiat_asset.price_usd.to_string()),
                    Some(fiat_asset.price_btc.to_string()),
                    None,
                    asset_percent_change_24h,
                    asset_percent_change_7d,
                    asset_percent_change_30d,
                    asset_percent_change_1y,
                    asset_percent_change_24h_btc,
                    Asset_Type::Fiat,
                    None,
                    None,
                    None,
                    None,
                    asset_last_updated_timestamp,
                    None,
                ),
            );
        }

        let client = match Creator::<Client>::create(environment_configuration.as_ref()).await {
            Ok(client_) => client_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        if let Err(mut error_auditor) = PostgresqlRepository::<Asset>::batch_upsert(
            &client,
            asset_registry_.as_slice(),
        )
        .await
        {
            error_auditor.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                ),
            );

            return Err(error_auditor);
        }

        return Ok(());
    }

    async fn request_fiat_asset_registry(environment_configuration: Arc<EnvironmentConfiguration>) -> Result<FiatAssetRegistry, Auditor<Error>> {
        let environment_configuration_ = environment_configuration.as_ref();

        let url = format!(
            "{}/api/v3/exchange_rates",
            environment_configuration_.remote_service.coingecko.pro.url.as_str()
        );

        let mut request_builder = match ReqwestClient::builder().build() {
            Ok(client_) => client_.get(url.as_str()),
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

        let header_value = match HeaderValue::from_str(environment_configuration_.remote_service.coingecko.pro.api_key.as_str()) {
            Ok(header_value_) => header_value_,
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

        request_builder = request_builder
            .header(
                ACCEPT,
                HeaderValue::from_static(Self::ACCEPT_HEADER_VALUE),
            )
            .header(
                Self::PRO_API_KEY_HEADER_KEY,
                header_value,
            );

        let response = match request_builder.send().await {
            Ok(response_) => response_,
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

        if response.status().is_client_error() || response.status().is_server_error() {
            let error_message = format!(
                coingecko_response_status_pattern!(),
                response.status().as_str(),
            );

            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new_(error_message.into()),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        let bytes = match response.bytes().await {
            Ok(bytes_) => bytes_,
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

        let fiat_asset_registry = match Serializer::<Json>::deserialize::<'_, FiatAssetRegistry>(bytes.chunk()) {
            Ok(fiat_asset_registry_) => fiat_asset_registry_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        return Ok(fiat_asset_registry);
    }

    fn checked_add_fiat_asset<'a>(
        fiat_asset_registry: &'a mut Vec<FiatAsset_>,
        fiat_asset_btc_price_usd: f64,
        fiat_asset_btc_price_target_currency: f64,
        fiat_asset_name: String,
        fiat_asset_symbol: &'static str,
    ) -> () {
        if fiat_asset_btc_price_target_currency.is_normal() {
            let fiat_asset_btc_price_usd_ = match Decimal::from_f64_retain(fiat_asset_btc_price_usd) {
                Some(fiat_asset_btc_price_usd__) => fiat_asset_btc_price_usd__,
                None => {
                    return ();
                }
            };

            let fiat_asset_btc_price_target_currency_ = match Decimal::from_f64_retain(fiat_asset_btc_price_target_currency) {
                Some(fiat_asset_btc_price_target_currency__) => fiat_asset_btc_price_target_currency__,
                None => {
                    return ();
                }
            };

            if fiat_asset_btc_price_target_currency_.is_zero() {
                return ();
            }

            let fiat_asset_target_currency_price_usd = fiat_asset_btc_price_usd_ / fiat_asset_btc_price_target_currency_;

            let fiat_asset_target_currency_price_btc = Decimal::ONE / fiat_asset_btc_price_target_currency_;

            fiat_asset_registry.push(
                FiatAsset_ {
                    id: fiat_asset_name.to_lowercase().replace(
                        " ",
                        "-",
                    ),
                    name: fiat_asset_name,
                    symbol: fiat_asset_symbol,
                    price_usd: fiat_asset_target_currency_price_usd,
                    price_btc: fiat_asset_target_currency_price_btc,
                },
            );
        }

        return ();
    }

    fn get_transformed_calculated_price_difference_percentage(price_from: Option<String>, price: Decimal) -> Option<String> {
        let price_difference_percentage = match price_from {
            Some(price_from_) => {
                let price_difference_percentage_ = match price_from_.parse::<f64>() {
                    Ok(price_from__) => {
                        let price_from___ = match Decimal::from_f64_retain(price_from__) {
                            Some(price_from____) => price_from____,
                            None => {
                                return None;
                            }
                        };

                        if price_from___.is_zero() {
                            return None;
                        }

                        let price_part = (price - price_from___) / price_from___;

                        Some(
                            (price_part * Decimal::ONE_HUNDRED)
                                .round_dp_with_strategy(
                                    3,
                                    RoundingStrategy::AwayFromZero,
                                )
                                .to_string(),
                        )
                    }
                    Err(_) => None,
                };

                price_difference_percentage_
            }
            None => None,
        };

        return price_difference_percentage;
    }
}

#[derive(Deserialize)]
struct CryptoAsset_ {
    id: String,
    symbol: String,
    name: String,
}

#[derive(Deserialize)]
struct DetailedCryptoAsset {
    id: String,
    symbol: String,
    name: String,
    image: Option<String>,
    current_price: Option<f64>,
    market_cap: Option<f64>,
    market_cap_rank: Option<f64>,
    fully_diluted_valuation: Option<f64>,
    total_volume: Option<f64>,
    high_24h: Option<f64>,
    low_24h: Option<f64>,
    price_change_24h: Option<f64>,
    price_change_percentage_24h: Option<f64>,
    market_cap_change_24h: Option<f64>,
    market_cap_change_percentage_24h: Option<f64>,
    circulating_supply: Option<f64>,
    total_supply: Option<f64>,
    max_supply: Option<f64>,
    ath: Option<f64>,
    ath_change_percentage: Option<f64>,
    ath_date: Option<String>,
    atl: Option<f64>,
    atl_change_percentage: Option<f64>,
    atl_date: Option<String>,
    roi: Option<Roi>,
    last_updated: Option<String>,
}

#[derive(Deserialize)]
struct Roi {
    times: Option<f64>,
    currency: Option<String>,
    percentage: Option<f64>,
}

#[derive(Deserialize)]
struct FiatAssetRegistry {
    rates: Rates,
}

#[derive(Deserialize)]
struct Rates {
    btc: FiatAsset,
    eth: FiatAsset,
    ltc: FiatAsset,
    bch: FiatAsset,
    bnb: FiatAsset,
    eos: FiatAsset,
    xrp: FiatAsset,
    xlm: FiatAsset,
    link: FiatAsset,
    dot: FiatAsset,
    yfi: FiatAsset,
    usd: FiatAsset,
    aed: FiatAsset,
    ars: FiatAsset,
    aud: FiatAsset,
    bdt: FiatAsset,
    bhd: FiatAsset,
    bmd: FiatAsset,
    brl: FiatAsset,
    cad: FiatAsset,
    chf: FiatAsset,
    clp: FiatAsset,
    cny: FiatAsset,
    czk: FiatAsset,
    dkk: FiatAsset,
    eur: FiatAsset,
    gbp: FiatAsset,
    gel: FiatAsset,
    hkd: FiatAsset,
    huf: FiatAsset,
    idr: FiatAsset,
    ils: FiatAsset,
    inr: FiatAsset,
    jpy: FiatAsset,
    krw: FiatAsset,
    kwd: FiatAsset,
    lkr: FiatAsset,
    mmk: FiatAsset,
    mxn: FiatAsset,
    myr: FiatAsset,
    ngn: FiatAsset,
    nok: FiatAsset,
    nzd: FiatAsset,
    php: FiatAsset,
    pkr: FiatAsset,
    pln: FiatAsset,
    rub: FiatAsset,
    sar: FiatAsset,
    sek: FiatAsset,
    sgd: FiatAsset,
    thb: FiatAsset,
    r#try: FiatAsset,
    twd: FiatAsset,
    uah: FiatAsset,
    vef: FiatAsset,
    vnd: FiatAsset,
    zar: FiatAsset,
    xdr: FiatAsset,
    xag: FiatAsset,
    xau: FiatAsset,
    bits: FiatAsset,
    sats: FiatAsset,
}

#[derive(Deserialize)]
struct FiatAsset {
    name: String,
    unit: String,
    value: f64,
    r#type: String,
}

struct FiatAsset_ {
    id: String,
    name: String,
    symbol: &'static str,
    price_usd: Decimal,
    price_btc: Decimal,
}

#[derive(Deserialize)]
struct AssetSnapshotForPriceDifferencePercentageCalculating {
    asset_id: String,
    price_usd_24_hours: String,
    price_btc_24_hours: Option<String>,
    price_usd_7_days: Option<String>,
    price_usd_30_days: Option<String>,
    price_usd_1_year: Option<String>,
}

struct AssetSnapshotForPriceDifferencePercentageCalculating_ {
    price_usd_24_hours: String,
    price_btc_24_hours: Option<String>,
    price_usd_7_days: Option<String>,
    price_usd_30_days: Option<String>,
    price_usd_1_year: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::FromPrimitive;
    use std::error::Error;

    #[test]
    fn get_transformed_calculated_price_difference_percentage() -> Result<(), Box<dyn Error + 'static>> {
        assert_eq!(
            Executor::<Task<UpdateAssets>>::get_transformed_calculated_price_difference_percentage(
                Some("100".to_string()),
                Decimal::from_u8(110).unwrap()
            ),
            Some("10.00".to_string()),
        );

        assert_eq!(
            Executor::<Task<UpdateAssets>>::get_transformed_calculated_price_difference_percentage(
                Some("100".to_string()),
                Decimal::from_u8(90).unwrap()
            ),
            Some("-10.00".to_string()),
        );

        assert_eq!(
            Executor::<Task<UpdateAssets>>::get_transformed_calculated_price_difference_percentage(
                Some("100".to_string()),
                Decimal::from_u8(100).unwrap()
            ),
            Some("0".to_string()),
        );

        assert_eq!(
            Executor::<Task<UpdateAssets>>::get_transformed_calculated_price_difference_percentage(
                None,
                Decimal::from_u8(100).unwrap()
            ),
            None,
        );

        assert_eq!(
            Executor::<Task<UpdateAssets>>::get_transformed_calculated_price_difference_percentage(
                Some("100".to_string()),
                Decimal::ZERO,
            ),
            Some("-100".to_string()),
        );

        return Ok(());
    }

    #[test]
    fn checked_add_fiat_asset() -> Result<(), Box<dyn Error + 'static>> {
        let mut fiat_asset_registry: Vec<FiatAsset_> = vec![];

        Executor::<Task<UpdateAssets>>::checked_add_fiat_asset(
            &mut fiat_asset_registry,
            100.0,
            1.0,
            String::new(),
            "",
        );

        assert_eq!(
            fiat_asset_registry[0].price_usd,
            Decimal::ONE_HUNDRED,
        );

        assert_eq!(
            fiat_asset_registry[0].price_btc,
            Decimal::ONE,
        );

        return Ok(());
    }
}

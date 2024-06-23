use super::Executor;
use crate::{
    application_layer::data::unified_report::UnifiedReport,
    domain_layer::{
        data::entity::{
            asset::Asset_Id,
            asset_snapshot::{
                AssetSnapshot,
                AssetSnapshot_1,
                AssetSnapshot_CreatedAt,
                AssetSnapshot_PriceBtc,
                AssetSnapshot_PriceUsd,
            },
            task::{
                generate_asset_snapshot::GenerateAssetSnapshot,
                Task,
            },
        },
        functionality::service::validator::Validator,
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
                converter::{
                    Convert,
                    Converter,
                },
                creator::Creator,
                resolver::Resolver,
                serializer::{
                    Serialize as _,
                    Serializer,
                },
            },
        },
    },
};
use bytes::Buf;
use http::{
    header::CONTENT_TYPE,
    HeaderValue,
};
use reqwest::{
    Body,
    Client as ReqwestClient,
};
use serde::Serialize;
use std::sync::Arc;
use tokio::{
    sync::Mutex,
    time::{
        sleep,
        Duration,
    },
};
use tokio_postgres::Client;

static QUANTITY_OF_CONSECUTIVE_TERMINATIONS_WITH_ERRORS: Mutex<usize> = Mutex::const_new(0);

impl Executor<Task<GenerateAssetSnapshot>> {
    const TASK_REPETITIONS_QUANTITY: u8 = 100;

    const TASK_SLEEPING_TIME_AFTER_ERROR_IN_SECONDS: u64 = 5;

    const LIMIT: i64 = 10000;

    pub async fn get_quantity_of_consecutive_terminations_with_errors() -> usize {
        return *(QUANTITY_OF_CONSECUTIVE_TERMINATIONS_WITH_ERRORS.lock().await);
    }

    pub async fn execute(environment_configuration: Arc<EnvironmentConfiguration>) -> Result<(), Auditor<Error>> {
        let mut offset_factor: i64 = 0;

        let mut should_execute = true;

        let mut counter: u8 = 0;

        'a: loop {
            if let Err(mut error_auditor) = Self::execute_(
                environment_configuration.clone(),
                &mut offset_factor,
                &mut should_execute,
            )
            .await
            {
                if counter < Self::TASK_REPETITIONS_QUANTITY {
                    counter += 1;

                    sleep(Duration::from_secs(Self::TASK_SLEEPING_TIME_AFTER_ERROR_IN_SECONDS)).await;

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

    async fn execute_<'a>(environment_configuration: Arc<EnvironmentConfiguration>, offset_factor: &'a mut i64, should_execute: &'a mut bool) -> Result<(), Auditor<Error>> {
        let asset_snapshot_created_at = match <Converter as Convert<i64, u32>>::convert(Resolver::<UTCDateTime>::get_now()) {
            Ok(asset_snapshot_created_at_) => AssetSnapshot_CreatedAt(asset_snapshot_created_at_),
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

        'a: while *should_execute {
            let offset = match (*offset_factor).checked_mul(Self::LIMIT) {
                Some(offset_) => offset_,
                None => {
                    return Err(
                        Auditor::<Error>::new(
                            Error::create_overflow_occured(),
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
            };

            let row_registry = match PostgresqlRepository::<AssetSnapshot_1>::get(
                &client,
                Self::LIMIT,
                offset,
            )
            .await
            {
                Ok(row_registry_) => row_registry_,
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

            if row_registry.is_empty() {
                return Ok(());
            }

            let row_registry_length = row_registry.len() as i64;

            let mut asset_snapshot_registry: Vec<AssetSnapshot> = vec![];

            'b: for row in row_registry.into_iter() {
                let asset_snapshot_asset_id = match row.try_get::<'_, usize, String>(0) {
                    Ok(asset_snapshot_asset_id_) => Asset_Id(asset_snapshot_asset_id_),
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

                let asset_snapshot_price_usd = match row.try_get::<'_, usize, String>(1) {
                    Ok(asset_snapshot_price_usd_) => AssetSnapshot_PriceUsd(asset_snapshot_price_usd_),
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

                let is_valid_asset_snapshot_price_usd = match Validator::<AssetSnapshot_PriceUsd>::is_valid(&asset_snapshot_price_usd) {
                    Ok(is_valid_asset_snapshot_price_usd_) => is_valid_asset_snapshot_price_usd_,
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

                if !is_valid_asset_snapshot_price_usd {
                    continue 'b;
                }

                let asset_snapshot_price_btc = match row.try_get::<'_, usize, Option<String>>(2) {
                    Ok(asset_snapshot_price_btc_) => {
                        let asset_snapshot_price_btc__ = match asset_snapshot_price_btc_ {
                            Some(asset_snapshot_price_btc___) => {
                                let asset_snapshot_price_btc____ = AssetSnapshot_PriceBtc(asset_snapshot_price_btc___);

                                let is_valid_asset_snapshot_price_btc = match Validator::<AssetSnapshot_PriceBtc>::is_valid(&asset_snapshot_price_btc____) {
                                    Ok(is_valid_asset_snapshot_price_btc_) => is_valid_asset_snapshot_price_btc_,
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

                                if !is_valid_asset_snapshot_price_btc {
                                    continue 'b;
                                }

                                Some(asset_snapshot_price_btc____)
                            }
                            None => None,
                        };

                        asset_snapshot_price_btc__
                    }
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

                asset_snapshot_registry.push(
                    AssetSnapshot {
                        asset_id: asset_snapshot_asset_id,
                        price_usd: asset_snapshot_price_usd,
                        price_btc: asset_snapshot_price_btc,
                        created_at: asset_snapshot_created_at,
                    },
                );
            }

            let unified_report = match Self::send_asset_snapshot_registry(
                environment_configuration.clone(),
                asset_snapshot_registry.as_slice(),
            )
            .await
            {
                Ok(unified_report_) => unified_report_,
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

            if unified_report.is_data_empty() {
                if row_registry_length < Self::LIMIT {
                    *should_execute = false;
                } else {
                    *offset_factor += 1;
                }

                continue 'a;
            } else {
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
        }

        return Ok(());
    }

    async fn send_asset_snapshot_registry<'a>(
        environment_configuration: Arc<EnvironmentConfiguration>,
        asset_snapshot_registry: &'a [AssetSnapshot],
    ) -> Result<UnifiedReport<Void, Void>, Auditor<Error>> {
        #[derive(Serialize)]
        struct RequestBodyData<'a> {
            server_access_token: &'a str,
            asset_snapshot_registry: &'a [AssetSnapshot],
        }

        let request_body_data = RequestBodyData {
            server_access_token: environment_configuration.as_ref().remote_service.data_proxy.server_access_token.as_str(),
            asset_snapshot_registry,
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
                    "{}/asset_snapshot/create",
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

        let unified_report = match Serializer::<Json>::deserialize::<UnifiedReport<Void, Void>>(response_body_data.chunk()) {
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

        return Ok(unified_report);
    }
}

use super::Executor;
use crate::{
    application_layer::data::unified_report::UnifiedReport,
    domain_layer::data::entity::task::{
        update_assets_for_subportfolio_trackable_wallet::UpdateAssetsForSubportfolioTrackableWallet,
        Task,
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            control_type::{
                Common,
                Json,
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
                logger::Logger,
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
use serde::{
    Deserialize,
    Serialize,
};
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

impl Executor<Task<UpdateAssetsForSubportfolioTrackableWallet>> {
    const TASK_REPETITIONS_QUANTITY: u16 = 250;

    const TASK_SLEEPING_TIME_AFTER_ERROR_IN_SECONDS_1: u64 = 1;

    const TASK_SLEEPING_TIME_AFTER_ERROR_IN_SECONDS_2: u64 = 10;

    const LIMIT: i16 = 250;

    pub async fn get_quantity_of_consecutive_terminations_with_errors() -> usize {
        return *(QUANTITY_OF_CONSECUTIVE_TERMINATIONS_WITH_ERRORS.lock().await);
    }

    pub async fn execute(environment_configuration: Arc<EnvironmentConfiguration>) -> Result<(), Auditor<Error>> {
        let mut requested_user_id: Option<i32> = None;

        let mut counter: u16 = 0;

        'a: loop {
            if let Err(mut error_auditor) = Self::execute_(
                environment_configuration.clone(),
                &mut requested_user_id,
            )
            .await
            {
                if counter < Self::TASK_REPETITIONS_QUANTITY {
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

    async fn execute_<'a>(environment_configuration: Arc<EnvironmentConfiguration>, requested_user_id: &'a mut Option<i32>) -> Result<(), Auditor<Error>> {
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

        'a: loop {
            let subportfolio_trackable_wallet_aggregated_registry = match Self::request_subportfolio_trackable_wallet_aggregated_registry(
                environment_configuration.clone(),
                *requested_user_id,
                Self::LIMIT,
            )
            .await
            {
                Ok(subportfolio_trackable_wallet_aggregated_registry_) => subportfolio_trackable_wallet_aggregated_registry_,
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

            let subportfolio_trackable_wallet_aggregated_registry_length = subportfolio_trackable_wallet_aggregated_registry.len();

            let mut new_requested_user_id = i32::MIN;

            '_b: for subportfolio_trackable_wallet_aggregated in subportfolio_trackable_wallet_aggregated_registry.into_iter() {
                let mut wallet_asset_registry: Vec<WalletAsset> = vec![];

                '_c: for wallet_id in subportfolio_trackable_wallet_aggregated.wallet_id_registry {
                    let row_registry = match PostgresqlRepository::<Common>::get_wallet_asset_registry(
                        &client,
                        subportfolio_trackable_wallet_aggregated.user_id,
                        wallet_id,
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

                    '_d: for row in row_registry.into_iter() {
                        let asset_id = match row.try_get::<'_, usize, String>(0) {
                            Ok(asset_id_) => asset_id_,
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

                        let asset_chain_id = match row.try_get::<'_, usize, Option<i32>>(1) {
                            Ok(asset_chain_id_) => asset_chain_id_,
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

                        let asset_network = match row.try_get::<'_, usize, Option<String>>(2) {
                            Ok(asset_network_) => asset_network_,
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

                        let wallet_address = match row.try_get::<'_, usize, String>(3) {
                            Ok(wallet_address_) => wallet_address_,
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

                        let wallet_label = match row.try_get::<'_, usize, Option<String>>(4) {
                            Ok(wallet_label_) => wallet_label_,
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

                        wallet_asset_registry.push(
                            WalletAsset {
                                wallet_id,
                                wallet_address,
                                wallet_label,
                                asset_network,
                                asset_chain_id,
                                asset_id,
                            },
                        );
                    }
                }

                Spawner::<TokioNonBlockingTask>::spawn_into_background(
                    Self::send_wallet_asset(
                        environment_configuration.clone(),
                        subportfolio_trackable_wallet_aggregated.user_id,
                        subportfolio_trackable_wallet_aggregated.subportfolio_id.clone(),
                        wallet_asset_registry,
                    ),
                );

                if new_requested_user_id < subportfolio_trackable_wallet_aggregated.user_id {
                    new_requested_user_id = subportfolio_trackable_wallet_aggregated.user_id;
                }
            }

            *requested_user_id = Some(new_requested_user_id + 1);

            if subportfolio_trackable_wallet_aggregated_registry_length < (Self::LIMIT as usize) {
                break 'a;
            }
        }

        return Ok(());
    }

    async fn request_subportfolio_trackable_wallet_aggregated_registry(
        environment_configuration: Arc<EnvironmentConfiguration>,
        user_id: Option<i32>,
        limit: i16,
    ) -> Result<Vec<SubportfolioTrackableWalletAggregated>, Auditor<Error>> {
        #[derive(Serialize)]
        struct RequestBodyData<'a> {
            server_access_token: &'a str,
            user_id: Option<i32>,
            limit: i16,
        }

        #[derive(Deserialize)]
        pub struct Outcoming {
            subportfolio_trackable_wallet_aggregated_registry: Vec<SubportfolioTrackableWalletAggregated>,
        }

        let request_body_data = RequestBodyData {
            server_access_token: environment_configuration.remote_service.data_proxy.server_access_token.as_str(),
            user_id,
            limit,
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
                    "{}/subportfolio_trackable_wallet/all",
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

        return Ok(outcoming.subportfolio_trackable_wallet_aggregated_registry);
    }

    async fn send_wallet_asset(
        environment_configuration: Arc<EnvironmentConfiguration>,
        user_id: i32,
        subportfolio_id: String,
        wallet_asset_registry: Vec<WalletAsset>,
    ) -> Result<(), Auditor<Error>> {
        'a: loop {
            if let Err(error_auditor) = Self::send_wallet_asset_(
                environment_configuration.clone(),
                user_id,
                subportfolio_id.as_str(),
                wallet_asset_registry.as_slice(),
            )
            .await
            {
                Logger::<Auditor<Error>>::log(&error_auditor);

                sleep(Duration::from_secs(Self::TASK_SLEEPING_TIME_AFTER_ERROR_IN_SECONDS_2)).await;

                continue 'a;
            }

            break 'a;
        }

        return Ok(());
    }

    async fn send_wallet_asset_<'a>(
        environment_configuration: Arc<EnvironmentConfiguration>,
        user_id: i32,
        subportfolio_id: &'a str,
        wallet_asset_registry: &'a [WalletAsset],
    ) -> Result<(), Auditor<Error>> {
        #[derive(Serialize)]
        struct RequestBodyData<'a> {
            server_access_token: &'a str,
            user_id: i32,
            subportfolio_id: &'a str,
            asset_registry: &'a [WalletAsset],
        }

        let request_body_data = RequestBodyData {
            server_access_token: environment_configuration.remote_service.data_proxy.server_access_token.as_str(),
            user_id,
            subportfolio_id,
            asset_registry: wallet_asset_registry,
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
                    "{}/subportfolio_asset/create_for_trackable_wallet",
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

        if response.status().is_server_error() {
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

        if !unified_report.is_data_empty() {
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

        return Ok(());
    }
}

#[derive(Deserialize)]
struct SubportfolioTrackableWalletAggregated {
    user_id: i32,
    subportfolio_id: String,
    wallet_id_registry: Vec<i32>,
}

#[derive(Serialize)]
struct WalletAsset {
    wallet_id: i32,
    wallet_address: String,
    wallet_label: Option<String>,
    asset_network: Option<String>,
    asset_chain_id: Option<i32>,
    asset_id: String,
}

use super::Executor;
use crate::{
    application_layer::data::unified_report::UnifiedReport,
    domain_layer::{
        data::entity::{
            _remote::{
                Asset_ChainId,
                Asset_Network,
                Exchange_Id,
                Exchange_Name,
                User_Id,
                Wallet_Address,
                Wallet_Id,
                Wallet_Label,
            },
            asset::Asset_Id,
            base_balance_snapshot::{
                BaseBalanceSnapshot,
                BaseBalanceSnapshot_1,
                BaseBalanceSnapshot_Amount,
                BaseBalanceSnapshot_CreatedAt,
            },
            task::{
                generate_base_balance_snapshot::GenerateBaseBalanceSnapshot,
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

impl Executor<Task<GenerateBaseBalanceSnapshot>> {
    const TASK_REPETITIONS_QUANTITY: u8 = 100;

    const TASK_SLEEPING_TIME_AFTER_ERROR_IN_SECONDS: u64 = 5;

    const LIMIT: i64 = 5000;

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
        let base_balance_snapshot_created_at = match <Converter as Convert<i64, u32>>::convert(Resolver::<UTCDateTime>::get_now()) {
            Ok(base_snapshot_created_at_) => BaseBalanceSnapshot_CreatedAt(base_snapshot_created_at_),
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

            let row_registry = match PostgresqlRepository::<BaseBalanceSnapshot_1>::get(
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

            let mut base_balance_snapshot_registry: Vec<BaseBalanceSnapshot> = vec![];

            'b: for row in row_registry.into_iter() {
                let base_balance_snapshot_user_id = match row.try_get::<'_, usize, i32>(0) {
                    Ok(base_balance_snapshot_user_id_) => User_Id(base_balance_snapshot_user_id_),
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

                let base_balance_snapshot_asset_id = match row.try_get::<'_, usize, String>(1) {
                    Ok(base_balance_snapshot_asset_id_) => Asset_Id(base_balance_snapshot_asset_id_),
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

                let base_balance_snapshot_exchange_id = match row.try_get::<'_, usize, Option<String>>(2) {
                    Ok(base_balance_snapshot_exchange_id_) => {
                        let base_balance_snapshot_exchange_id__ = match base_balance_snapshot_exchange_id_ {
                            Some(base_balance_snapshot_exchange_id___) => Some(Exchange_Id(base_balance_snapshot_exchange_id___)),
                            None => None,
                        };

                        base_balance_snapshot_exchange_id__
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

                let base_balance_snapshot_wallet_id = match row.try_get::<'_, usize, Option<i32>>(3) {
                    Ok(base_balance_snapshot_wallet_id_) => {
                        let base_balance_snapshot_wallet_id__ = match base_balance_snapshot_wallet_id_ {
                            Some(base_balance_snapshot_wallet_id___) => Some(Wallet_Id(base_balance_snapshot_wallet_id___)),
                            None => None,
                        };

                        base_balance_snapshot_wallet_id__
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

                let base_balance_snapshot_asset_chain_id = match row.try_get::<'_, usize, Option<i32>>(4) {
                    Ok(base_balance_snapshot_asset_chain_id_) => {
                        let base_balance_snapshot_asset_chain_id__ = match base_balance_snapshot_asset_chain_id_ {
                            Some(base_balance_snapshot_asset_chain_id___) => Some(Asset_ChainId(base_balance_snapshot_asset_chain_id___)),
                            None => None,
                        };

                        base_balance_snapshot_asset_chain_id__
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

                let base_balance_snapshot_asset_network = match row.try_get::<'_, usize, Option<String>>(5) {
                    Ok(base_balance_snapshot_asset_network_) => {
                        let base_balance_snapshot_asset_network__ = match base_balance_snapshot_asset_network_ {
                            Some(base_balance_snapshot_asset_network___) => Some(Asset_Network(base_balance_snapshot_asset_network___)),
                            None => None,
                        };

                        base_balance_snapshot_asset_network__
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

                let base_balance_snapshot_amount = match row.try_get::<'_, usize, Option<String>>(6) {
                    Ok(base_balance_snapshot_amount_) => {
                        let base_balance_snapshot_amount__ = match base_balance_snapshot_amount_ {
                            Some(base_balance_snapshot_amount___) => BaseBalanceSnapshot_Amount(base_balance_snapshot_amount___),
                            None => {
                                continue 'b;
                            }
                        };

                        base_balance_snapshot_amount__
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

                let is_base_balance_snapshot_amount = match Validator::<BaseBalanceSnapshot_Amount>::is_valid(&base_balance_snapshot_amount) {
                    Ok(base_balance_snapshot_amount_) => base_balance_snapshot_amount_,
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

                if !is_base_balance_snapshot_amount {
                    continue 'b;
                }

                let base_balance_snapshot_wallet_address = match row.try_get::<'_, usize, Option<String>>(7) {
                    Ok(base_balance_snapshot_wallet_address_) => {
                        let base_balance_snapshot_wallet_address__ = match base_balance_snapshot_wallet_address_ {
                            Some(base_balance_snapshot_wallet_address___) => Some(Wallet_Address(base_balance_snapshot_wallet_address___)),
                            None => None,
                        };

                        base_balance_snapshot_wallet_address__
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

                let base_balance_snapshot_wallet_label = match row.try_get::<'_, usize, Option<String>>(8) {
                    Ok(base_balance_snapshot_wallet_label_) => {
                        let base_balance_snapshot_wallet_label__ = match base_balance_snapshot_wallet_label_ {
                            Some(base_balance_snapshot_wallet_label___) => Some(Wallet_Label(base_balance_snapshot_wallet_label___)),
                            None => None,
                        };

                        base_balance_snapshot_wallet_label__
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

                let base_balance_snapshot_exchange_name = match row.try_get::<'_, usize, Option<String>>(9) {
                    Ok(base_balance_snapshot_exchange_name_) => {
                        let base_balance_snapshot_exchange_name__ = match base_balance_snapshot_exchange_name_ {
                            Some(base_balance_snapshot_exchange_name___) => Some(Exchange_Name(base_balance_snapshot_exchange_name___)),
                            None => None,
                        };

                        base_balance_snapshot_exchange_name__
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

                base_balance_snapshot_registry.push(
                    BaseBalanceSnapshot {
                        user_id: base_balance_snapshot_user_id,
                        asset_id: base_balance_snapshot_asset_id,
                        asset_chain_id: base_balance_snapshot_asset_chain_id,
                        asset_network: base_balance_snapshot_asset_network,
                        wallet_id: base_balance_snapshot_wallet_id,
                        wallet_address: base_balance_snapshot_wallet_address,
                        wallet_label: base_balance_snapshot_wallet_label,
                        exchange_id: base_balance_snapshot_exchange_id,
                        exchange_name: base_balance_snapshot_exchange_name,
                        amount: base_balance_snapshot_amount,
                        created_at: base_balance_snapshot_created_at,
                    },
                );
            }

            let unified_report = match Self::send_base_balance_snapshot_registry(
                environment_configuration.clone(),
                base_balance_snapshot_registry.as_slice(),
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

    async fn send_base_balance_snapshot_registry<'a>(
        environment_configuration: Arc<EnvironmentConfiguration>,
        base_balance_snapshot_registry: &'a [BaseBalanceSnapshot],
    ) -> Result<UnifiedReport<Void, Void>, Auditor<Error>> {
        #[derive(Serialize)]
        struct RequestBodyData<'a> {
            server_access_token: &'a str,
            base_balance_snapshot_registry: &'a [BaseBalanceSnapshot],
        }

        let request_body_data = RequestBodyData {
            server_access_token: environment_configuration.as_ref().remote_service.data_proxy.server_access_token.as_str(),
            base_balance_snapshot_registry,
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
                    "{}/base_balance_snapshot/create",
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

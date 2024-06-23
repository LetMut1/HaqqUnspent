use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::control_type::Json;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::functionality::service::serializer::Serialize as _;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use bytes::Buf;
use reqwest::header::HeaderValue;
use reqwest::header::ACCEPT;
use reqwest::Client;
use reqwest::StatusCode;
use serde::Deserialize;

use super::HttpRequestResolver;

pub use crate::infrastructure_layer::data::control_type::EVM;

impl HttpRequestResolver<EVM> {
    pub async fn get_transaction_registry_by_address<'a>(evm_address: &'a str) -> Result<EvmTransactionRegistryByAddressResponse, Auditor<Error>> {
        let url = format!(
            "https://explorer.haqq.network/api/v2/addresses/{}/transactions",
            evm_address,
        );

        let mut request_builder = match Client::builder().build() {
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

        request_builder = request_builder.header(
            ACCEPT,
            HeaderValue::from_static("application/json"),
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

        if response.status() == StatusCode::OK {
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

            let item_registry = match Serializer::<Json>::deserialize::<'_, ItemRegistry>(bytes.chunk()) {
                Ok(item_registry_) => item_registry_,
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

            return Ok(
                EvmTransactionRegistryByAddressResponse::ItemRegistry {
                    item_registry,
                },
            );
        }

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(EvmTransactionRegistryByAddressResponse::NotFound);
        }

        return Err(
            Auditor::<Error>::new(
                Error::Runtime {
                    runtime: Runtime::Other {
                        other: Other::new_("Response status is not 200.".into()),
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

pub enum EvmTransactionRegistryByAddressResponse {
    ItemRegistry {
        item_registry: ItemRegistry,
    },
    NotFound,
}

#[derive(Deserialize)]
pub struct ItemRegistry {
    pub items: Vec<Item>,
}

#[derive(Deserialize)]
pub struct Item {
    pub status: String,
    pub confirmations: i64,
    pub result: String,
    pub from: From,
    pub tx_types: Vec<String>,
    pub value: String,
}

#[derive(Deserialize)]
pub struct From {
    pub hash: String,
}

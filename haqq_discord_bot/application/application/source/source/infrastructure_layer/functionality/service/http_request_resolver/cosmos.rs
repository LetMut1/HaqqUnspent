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
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

use super::HttpRequestResolver;

pub use crate::infrastructure_layer::data::control_type::Cosmos;

impl HttpRequestResolver<Cosmos> {
    pub async fn get_aislm_balance_by_address<'a>(bech32_address: &'a str) -> Result<Balance, Auditor<Error>> {
        let url = format!(
            "https://sdk.haqq.sh/cosmos/bank/v1beta1/balances/{}/by_denom?denom=aISLM",
            bech32_address,
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

        if !response.status().is_success() {
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

        let balance = match Serializer::<Json>::deserialize::<'_, Balance>(bytes.chunk()) {
            Ok(balance_) => balance_,
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

        return Ok(balance);
    }

    pub async fn get_aislm_stake_quantity_by_address<'a>(bech32_address: &'a str) -> Result<StakeQuantityByAddressResponse, Auditor<Error>> {
        let url = format!(
            "https://sdk.haqq.sh/cosmos/staking/v1beta1/delegations/{}",
            bech32_address,
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
            CONTENT_TYPE,
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

        if !response.status().is_success() {
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

        let stake_quantity_by_address_response = match Serializer::<Json>::deserialize::<'_, StakeQuantityByAddressResponse>(bytes.chunk()) {
            Ok(stake_quantity_by_address_response_) => stake_quantity_by_address_response_,
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

        return Ok(stake_quantity_by_address_response);
    }

    pub async fn get_transaction_registry_by_transfer_recipient_address<'a>(bech32_address: &'a str) -> Result<CosmosTransactionRegistryByAddressResponse, Auditor<Error>> {
        let url = format!(
            "https://sdk.haqq.sh/cosmos/tx/v1beta1/txs?events=transfer.recipient=\'{}\'&pagination.limit=50&pagination.count_total=true&pagination.reverse=true",
            bech32_address,
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

        if !response.status().is_success() {
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

        let cosmos_transaction_registry_by_address_response = match Serializer::<Json>::deserialize::<'_, CosmosTransactionRegistryByAddressResponse>(bytes.chunk()) {
            Ok(cosmos_transaction_registry_by_address_response_) => cosmos_transaction_registry_by_address_response_,
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

        return Ok(cosmos_transaction_registry_by_address_response);
    }
}

#[derive(Deserialize)]
pub struct Balance {
    pub balance: Balance_1,
}

#[derive(Deserialize)]
pub struct Balance_1 {
    pub denom: String,
    pub amount: String,
}

#[derive(Deserialize)]
pub struct StakeQuantityByAddressResponse {
    pub delegation_responses: Vec<DelegationResponse>,
}

#[derive(Deserialize)]
pub struct DelegationResponse {
    pub balance: Balance_2,
}

#[derive(Deserialize)]
pub struct Balance_2 {
    pub amount: String,
}

#[derive(Deserialize)]
pub struct CosmosTransactionRegistryByAddressResponse {
    pub txs: Vec<Transaction>,
    // pub pagination: Value,
    // pub total: String,
}

#[derive(Deserialize)]
pub struct Transaction {
    pub body: Body,
}

#[derive(Deserialize)]
pub struct Body {
    pub messages: Vec<Value>,
}

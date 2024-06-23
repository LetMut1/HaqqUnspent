use super::Resolver;
pub use crate::infrastructure_layer::data::control_type::AccessToken;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        control_type::Json,
        environment_configuration::ENVIRONMENT_CONFIGURATION,
        error::{
            Error,
            Other,
            Runtime,
        },
    },
    functionality::service::serializer::{
        Serialize,
        Serializer,
    },
};
use bytes::Buf;
use http::{
    header::ACCEPT,
    HeaderValue,
};
use reqwest::Client;
use serde::Deserialize;

impl Resolver<AccessToken> {
    pub async fn get_user<'a>(access_token: &'a AccessToken) -> Result<Option<User>, Auditor<Error>> {
        let mut request_builder = match Client::builder().build() {
            Ok(client_) => client_.get(ENVIRONMENT_CONFIGURATION.remote_service.user_authorization.url.0),
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
                HeaderValue::from_static("application/json"),
            )
            .bearer_auth(access_token.0.as_str());

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
            return Ok(None);
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

        let user = match Serializer::<Json>::deserialize::<'_, User>(bytes.chunk()) {
            Ok(data_) => data_,
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

        return Ok(Some(user));
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    // The data type is Option<T>, because it is difficult to understand it from the service api description.
    // This must be determined at the moment when this data is needed.
    pub email: Option<String>,
    // The data type is Option<T>, because it is difficult to understand it from the service api description.
    // This must be determined at the moment when this data is needed.
    pub is_onboarding_checklist_completed: Option<bool>,
    // The data type is Option<T>, because it is difficult to understand it from the service api description.
    // This must be determined at the moment when this data is needed.
    pub email_pref_newsletter: Option<bool>,
    // The data type is Option<T>, because it is difficult to understand it from the service api description.
    // This must be determined at the moment when this data is needed.
    pub email_pref_curated_content: Option<bool>,
    // The data type is Option<T>, because it is difficult to understand it from the service api description.
    // This must be determined at the moment when this data is needed.
    pub email_validity_status: Option<String>,
    // The data type is Option<T>, because it is difficult to understand it from the service api description.
    // This must be determined at the moment when this data is needed.
    pub reference_asset_id: Option<String>,
    // The data type is Option<T>, because it is difficult to understand it from the service api description.
    // This must be determined at the moment when this data is needed.
    pub plan: Option<String>,
    // The data type is Option<T>, because it is difficult to understand it from the service api description.
    // This must be determined at the moment when this data is needed.
    pub is_trialist: Option<bool>,
    // The data type is Option<T>, because it is difficult to understand it from the service api description.
    // This must be determined at the moment when this data is needed.
    pub is_payment_retry_period: Option<bool>,
    // The data type is Option<T>, because it is difficult to understand it from the service api description.
    // This must be determined at the moment when this data is needed.
    pub customer_id: Option<String>,
    // The data type is Option<T>, because it is difficult to understand it from the service api description.
    // This must be determined at the moment when this data is needed.
    pub is_email_confirmed: Option<bool>,
    // The data type is Option<T>, because it is difficult to understand it from the service api description.
    // This must be determined at the moment when this data is needed.
    pub is_email_allowed_user: Option<bool>,
}

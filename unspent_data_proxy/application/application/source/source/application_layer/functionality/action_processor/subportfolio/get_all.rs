#[cfg(not(feature = "not_authorized_user"))]
use crate::infrastructure_layer::data::control_type::AccessToken;
pub use crate::infrastructure_layer::data::control_type::Subportfolio___GetAll;
#[cfg(not(feature = "not_authorized_user"))]
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
#[cfg(feature = "not_authorized_user")]
use crate::infrastructure_layer::functionality::service::resolver::access_token::User;
#[cfg(not(feature = "not_authorized_user"))]
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::data::entity::subportfolio::Subportfolio_1,
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            error::Error,
            invalid_argument_result::InvalidArgumentResult,
            void::Void,
        },
        functionality::repository::clickhouse::{
            by::By2,
            ClickhouseRepository,
        },
    },
};
use clickhouse::Client;
use serde::{
    Deserialize,
    Serialize,
};

impl ActionProcessor<Subportfolio___GetAll> {
    pub async fn process(incoming: Option<Incoming>, clickhouse_client: Client) -> Result<InvalidArgumentResult<UnifiedReport<Outcoming, Void>>, Auditor<Error>> {
        let incoming_ = match incoming {
            Some(incoming__) => incoming__,
            None => {
                return Err(
                    Auditor::<Error>::new(
                        Error::create_incoming_invalid_state(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let user_;

        #[cfg(feature = "not_authorized_user")]
        {
            user_ = incoming_.user;
        }

        #[cfg(not(feature = "not_authorized_user"))]
        {
            let user = match Resolver::<AccessToken>::get_user(&incoming_.access_token).await {
                Ok(user_) => user_,
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

            user_ = match user {
                Some(user__) => user__,
                None => {
                    return Ok(
                        InvalidArgumentResult::InvalidArgumentAuditor {
                            invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                                InvalidArgument::new(),
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                ),
                            ),
                        },
                    );
                }
            };
        }

        let subportfolio_1_registry = match ClickhouseRepository::<Subportfolio_1>::get_all(
            &clickhouse_client,
            By2 {
                user_id: user_.id,
            },
        )
        .await
        {
            Ok(subportfolio_1_registry_) => subportfolio_1_registry_,
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
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::filled(
                    Outcoming {
                        subportfolio_registry: subportfolio_1_registry,
                    },
                ),
            },
        );
    }
}

#[cfg(feature = "not_authorized_user")]
#[derive(Deserialize)]
pub struct Incoming {
    user: User,
}

#[cfg(not(feature = "not_authorized_user"))]
#[derive(Deserialize)]
pub struct Incoming {
    access_token: AccessToken,
}

#[derive(Serialize)]
pub struct Outcoming {
    subportfolio_registry: Vec<Subportfolio_1>,
}

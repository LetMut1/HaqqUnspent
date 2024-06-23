#[cfg(not(feature = "not_authorized_user"))]
use crate::infrastructure_layer::data::control_type::AccessToken;
pub use crate::infrastructure_layer::data::control_type::SubportfolioTrackableWallet___GetAllForSubportfolio;
#[cfg(feature = "not_authorized_user")]
use crate::infrastructure_layer::functionality::service::resolver::access_token::User;
#[cfg(not(feature = "not_authorized_user"))]
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use crate::{
    application_layer::{
        data::unified_report::{
            CommonPrecedent,
            UnifiedReport,
        },
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::{
            subportfolio::{
                Subportfolio,
                Subportfolio_Id,
            },
            subportfolio_trackable_wallet::SubportfolioTrackableWallet_1,
        },
        functionality::service::validator::Validator,
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            error::{
                Error,
                Other,
                Runtime,
            },
            invalid_argument::InvalidArgument,
            invalid_argument_result::InvalidArgumentResult,
        },
        functionality::repository::clickhouse::{
            by::By3,
            ClickhouseRepository,
        },
        macro_rules::r#enum,
    },
};
use clickhouse::Client;
use serde::{
    Deserialize,
    Serialize,
};

impl ActionProcessor<SubportfolioTrackableWallet___GetAllForSubportfolio> {
    pub async fn process(incoming: Option<Incoming>, clickhouse_client: Client) -> Result<InvalidArgumentResult<UnifiedReport<Outcoming, Precedent>>, Auditor<Error>> {
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

        if !Validator::<Subportfolio_Id>::is_valid(incoming_.subportfolio_id.as_str()) {
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

        let by_3 = By3 {
            user_id: user_.id,
            subportfolio_id: incoming_.subportfolio_id.as_str(),
        };

        let is_exist = match ClickhouseRepository::<Subportfolio>::is_exist_1(
            &clickhouse_client,
            &by_3,
        )
        .await
        {
            Ok(is_exist_) => is_exist_,
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

        if !is_exist {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::Subportfolio_DoesNotExist),
                },
            );
        }

        let mut subportfolio_trackable_wallet_1_row_cursor = match ClickhouseRepository::<SubportfolioTrackableWallet_1>::get_(
            &clickhouse_client,
            &by_3,
        ) {
            Ok(subportfolio_trackable_wallet_1_row_cursor_) => subportfolio_trackable_wallet_1_row_cursor_,
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

        let mut subportfolio_trackable_wallet_wallet_id_registry: Vec<i32> = vec![];

        'a: loop {
            match subportfolio_trackable_wallet_1_row_cursor.next().await {
                Ok(subportfolio_trackable_wallet_1) => {
                    match subportfolio_trackable_wallet_1 {
                        Some(subportfolio_trackable_wallet_1_) => {
                            subportfolio_trackable_wallet_wallet_id_registry.push(subportfolio_trackable_wallet_1_.wallet_id);
                        }
                        None => {
                            break 'a;
                        }
                    }
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
            }
        }

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::filled(
                    Outcoming {
                        subportfolio_trackable_wallet_wallet_id_registry,
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
    subportfolio_id: String,
}

#[cfg(not(feature = "not_authorized_user"))]
#[derive(Deserialize)]
pub struct Incoming {
    access_token: AccessToken,
    subportfolio_id: String,
}

#[derive(Serialize)]
pub struct Outcoming {
    subportfolio_trackable_wallet_wallet_id_registry: Vec<i32>,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::Subportfolio_DoesNotExist,
    }
);

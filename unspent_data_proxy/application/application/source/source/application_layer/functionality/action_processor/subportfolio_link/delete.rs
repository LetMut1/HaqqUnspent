#[cfg(not(feature = "not_authorized_user"))]
use crate::infrastructure_layer::data::control_type::AccessToken;
pub use crate::infrastructure_layer::data::control_type::SubportfolioLink___Delete;
#[cfg(feature = "not_authorized_user")]
use crate::infrastructure_layer::functionality::service::resolver::access_token::User;
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
                IsDeleted,
                Subportfolio_Id,
            },
            subportfolio_link::{
                SubportfolioLink,
                SubportfolioLink_Id,
            },
        },
        functionality::service::{
            creator::Creator,
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            control_type::UTCDateTime,
            error::Error,
            invalid_argument::InvalidArgument,
            invalid_argument_result::InvalidArgumentResult,
            void::Void,
        },
        functionality::{
            repository::clickhouse::{
                by::By8,
                ClickhouseRepository,
            },
            service::resolver::Resolver,
        },
        macro_rules::r#enum,
    },
};
use clickhouse::Client;
use serde::Deserialize;

impl ActionProcessor<SubportfolioLink___Delete> {
    pub async fn process(incoming: Option<Incoming>, clickhouse_client: Client) -> Result<InvalidArgumentResult<UnifiedReport<Void, Precedent>>, Auditor<Error>> {
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

        if !Validator::<SubportfolioLink_Id>::is_valid(incoming_.subportfolio_link_id.as_str()) {
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

        let is_exist = match ClickhouseRepository::<SubportfolioLink>::is_exist_1(
            &clickhouse_client,
            &By8 {
                subportfolio_link_id: incoming_.subportfolio_link_id.as_str(),
                user_id: user_.id,
            },
        )
        .await
        {
            Ok(subportfolio_link_1_) => subportfolio_link_1_,
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
                    subject: UnifiedReport::precedent(Precedent::SubportfolioLink_DoesNotExist),
                },
            );
        }

        let subportfolio_link_updated_at = match Resolver::<UTCDateTime>::get_now_() {
            Ok(subportfolio_updated_at_) => subportfolio_updated_at_,
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

        let subportfolio_link = SubportfolioLink::new(
            incoming_.subportfolio_link_id,
            user_.id,
            Creator::<Subportfolio_Id>::create_minimum_length(),
            false,
            None,
            subportfolio_link_updated_at,
            subportfolio_link_updated_at,
            IsDeleted::create_deleted().get(),
        );

        if let Err(mut error_auditor) = ClickhouseRepository::<SubportfolioLink>::create(
            &clickhouse_client,
            &subportfolio_link,
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
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::empty(),
            },
        );
    }
}

#[cfg(feature = "not_authorized_user")]
#[derive(Deserialize)]
pub struct Incoming {
    user: User,
    subportfolio_link_id: String,
}

#[cfg(not(feature = "not_authorized_user"))]
#[derive(Deserialize)]
pub struct Incoming {
    access_token: AccessToken,
    subportfolio_link_id: String,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::SubportfolioLink_DoesNotExist,
    }
);

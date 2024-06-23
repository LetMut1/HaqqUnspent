#[cfg(not(feature = "not_authorized_user"))]
use crate::infrastructure_layer::data::control_type::AccessToken;
pub use crate::infrastructure_layer::data::control_type::SubportfolioLink___Create;
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
                Subportfolio,
                Subportfolio_Id,
            },
            subportfolio_link::{
                SubportfolioLink,
                SubportfolioLink_Description,
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
            error::Error,
            invalid_argument::InvalidArgument,
            invalid_argument_result::InvalidArgumentResult,
        },
        functionality::{
            repository::clickhouse::{
                by::By3,
                ClickhouseRepository,
            },
            service::resolver::{
                utc_date_time::UTCDateTime,
                Resolver,
            },
        },
        macro_rules::r#enum,
    },
};
use clickhouse::Client;
use serde::{
    Deserialize,
    Serialize,
};

impl ActionProcessor<SubportfolioLink___Create> {
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

        if let Some(ref subportfolio_link_description) = incoming_.subportfolio_link_description {
            let is_valid_subportfolio_link_description = match Validator::<SubportfolioLink_Description>::is_valid(subportfolio_link_description.as_str()) {
                Ok(is_valid_subportfolio_link_description_) => is_valid_subportfolio_link_description_,
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

            if !is_valid_subportfolio_link_description {
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

        let subportfolio_link_quantity = match ClickhouseRepository::<SubportfolioLink>::get_count(
            &clickhouse_client,
            &by_3,
        )
        .await
        {
            Ok(subportfolio_link_quantity_) => subportfolio_link_quantity_,
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

        if (subportfolio_link_quantity + 1) > SubportfolioLink::MAXIMUM_QUANTITY_PER_USER_AND_SUBPORTFOLIO {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::SubportfolioLink_MaximumQuantityPerUserAndSubportfolio),
                },
            );
        }

        let subportfolio_link_id = Creator::<SubportfolioLink_Id>::create();

        let subportfolio_link_created_at = match Resolver::<UTCDateTime>::get_now_() {
            Ok(subportfolio_link_created_at_) => subportfolio_link_created_at_,
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
            subportfolio_link_id,
            user_.id,
            incoming_.subportfolio_id,
            true,
            incoming_.subportfolio_link_description,
            subportfolio_link_created_at,
            subportfolio_link_created_at,
            IsDeleted::create_not_deleted().get(),
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
                subject: UnifiedReport::filled(
                    Outcoming {
                        subportfolio_link_id: subportfolio_link.id,
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
    subportfolio_link_description: Option<String>,
}

#[cfg(not(feature = "not_authorized_user"))]
#[derive(Deserialize)]
pub struct Incoming {
    access_token: AccessToken,
    subportfolio_id: String,
    subportfolio_link_description: Option<String>,
}

#[derive(Serialize)]
pub struct Outcoming {
    subportfolio_link_id: String,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::Subportfolio_DoesNotExist,
        CommonPrecedent::SubportfolioLink_MaximumQuantityPerUserAndSubportfolio,
    }
);

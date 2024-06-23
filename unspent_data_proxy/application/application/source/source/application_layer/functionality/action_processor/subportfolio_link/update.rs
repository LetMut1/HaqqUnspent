#[cfg(not(feature = "not_authorized_user"))]
use crate::infrastructure_layer::data::control_type::AccessToken;
pub use crate::infrastructure_layer::data::control_type::SubportfolioLink___Update;
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
            subportfolio::IsDeleted,
            subportfolio_link::{
                SubportfolioLink,
                SubportfolioLink_2,
                SubportfolioLink_Description,
                SubportfolioLink_Id,
            },
        },
        functionality::service::validator::Validator,
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            error::Error,
            invalid_argument::InvalidArgument,
            invalid_argument_result::InvalidArgumentResult,
            serialization_layer::SerializationLayer,
            void::Void,
        },
        functionality::{
            repository::clickhouse::{
                by::By8,
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
use serde::Deserialize;

impl ActionProcessor<SubportfolioLink___Update> {
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

        if incoming_.subportfolio_link_is_active.is_none() && incoming_.subportfolio_link_description.is_none() {
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

        if let Some(ref serialization_layer) = incoming_.subportfolio_link_description {
            if let Some(ref subportfolio_link_description) = serialization_layer.data {
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
        }

        let subportfolio_link_2 = match ClickhouseRepository::<SubportfolioLink_2>::find(
            &clickhouse_client,
            &By8 {
                subportfolio_link_id: incoming_.subportfolio_link_id.as_str(),
                user_id: user_.id,
            },
        )
        .await
        {
            Ok(subportfolio_link_2_) => subportfolio_link_2_,
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

        let subportfolio_link_2_ = match subportfolio_link_2 {
            Some(subportfolio_2__) => subportfolio_2__,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::SubportfolioLink_DoesNotExist),
                    },
                );
            }
        };

        let subportfolio_link_is_active = match incoming_.subportfolio_link_is_active {
            Some(subportfolio_link_is_active_) => subportfolio_link_is_active_,
            None => subportfolio_link_2_.is_active,
        };

        let subportfolio_link_description = match incoming_.subportfolio_link_description {
            Some(serialization_layer) => serialization_layer.data,
            None => subportfolio_link_2_.description,
        };

        let subportfolio_link_updatet_at = match Resolver::<UTCDateTime>::get_now_() {
            Ok(subportfolio_link_updatet_at_) => subportfolio_link_updatet_at_,
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
            subportfolio_link_2_.subportfolio_id,
            subportfolio_link_is_active,
            subportfolio_link_description,
            subportfolio_link_2_.created_at,
            subportfolio_link_updatet_at,
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
    subportfolio_link_is_active: Option<bool>,
    subportfolio_link_description: Option<SerializationLayer<Option<String>>>,
}

#[cfg(not(feature = "not_authorized_user"))]
#[derive(Deserialize)]
pub struct Incoming {
    access_token: AccessToken,
    subportfolio_link_id: String,
    subportfolio_link_is_active: Option<bool>,
    subportfolio_link_description: Option<SerializationLayer<Option<String>>>,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::SubportfolioLink_DoesNotExist
    }
);

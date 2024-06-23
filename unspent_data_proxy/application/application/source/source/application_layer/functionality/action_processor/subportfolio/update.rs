#[cfg(not(feature = "not_authorized_user"))]
use crate::infrastructure_layer::data::control_type::AccessToken;
pub use crate::infrastructure_layer::data::control_type::Subportfolio___Update;
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
            _remote::User_Id,
            subportfolio::{
                IsDeleted,
                Subportfolio,
                Subportfolio_2,
                Subportfolio_Description,
                Subportfolio_Id,
                Subportfolio_Name,
                Subportfolio_UpdatedAt,
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
                by::{
                    By3,
                    By7,
                },
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

impl ActionProcessor<Subportfolio___Update> {
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

        if incoming_.subportfolio_name.is_none() && incoming_.subportfolio_description.is_none() {
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

        if !Validator::<Subportfolio_Id>::is_valid(incoming_.subportfolio_id.0.as_str()) {
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

        if let Some(ref subportfolio_name) = incoming_.subportfolio_name {
            let is_valid_subportfolio_name = match Validator::<Subportfolio_Name>::is_valid(subportfolio_name.0.as_str()) {
                Ok(is_valid_subportfolio_name_) => is_valid_subportfolio_name_,
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

            if !is_valid_subportfolio_name {
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

        if let Some(ref serialization_layer) = incoming_.subportfolio_description {
            if let Some(ref subportfolio_description) = serialization_layer.data {
                let is_valid_subportfolio_description = match Validator::<Subportfolio_Description>::is_valid(subportfolio_description.0.as_str()) {
                    Ok(is_valid_subportfolio_description_) => is_valid_subportfolio_description_,
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

                if !is_valid_subportfolio_description {
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

        let subportfolio_2 = match ClickhouseRepository::<Subportfolio_2>::find(
            &clickhouse_client,
            &By3 {
                user_id: user_.id,
                subportfolio_id: incoming_.subportfolio_id.0.as_str(),
            },
        )
        .await
        {
            Ok(subportfolio_2_) => subportfolio_2_,
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

        let subportfolio_2_ = match subportfolio_2 {
            Some(subportfolio_2__) => subportfolio_2__,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::Subportfolio_DoesNotExist),
                    },
                );
            }
        };

        let subportfolio_name = match incoming_.subportfolio_name {
            Some(subportfolio_name_) => {
                let is_exist = match ClickhouseRepository::<Subportfolio>::is_exist_2(
                    &clickhouse_client,
                    &By7 {
                        user_id: user_.id,
                        subportfolio_name: subportfolio_name_.0.as_str(),
                    },
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

                if is_exist {
                    return Ok(
                        InvalidArgumentResult::Ok {
                            subject: UnifiedReport::precedent(Precedent::Subportfolio_AlreadyExists),
                        },
                    );
                }

                subportfolio_name_
            }
            None => subportfolio_2_.name,
        };

        let subportfolio_description = match incoming_.subportfolio_description {
            Some(serialization_layer) => serialization_layer.data,
            None => subportfolio_2_.description,
        };

        let subportfolio_updatet_at = match Resolver::<UTCDateTime>::get_now_() {
            Ok(subportfolio_updatet_at_) => subportfolio_updatet_at_,
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

        let subportfolio = Subportfolio {
            user_id: User_Id(user_.id),
            id: incoming_.subportfolio_id,
            name: subportfolio_name,
            description: subportfolio_description,
            created_at: subportfolio_2_.created_at,
            updated_at: Subportfolio_UpdatedAt(subportfolio_updatet_at),
            is_deleted: IsDeleted::create_not_deleted(),
        };

        if let Err(mut error_auditor) = ClickhouseRepository::<Subportfolio>::create(
            &clickhouse_client,
            &subportfolio,
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
    subportfolio_id: Subportfolio_Id,
    subportfolio_name: Option<Subportfolio_Name>,
    subportfolio_description: Option<SerializationLayer<Option<Subportfolio_Description>>>,
}

#[cfg(not(feature = "not_authorized_user"))]
#[derive(Deserialize)]
pub struct Incoming {
    access_token: AccessToken,
    subportfolio_id: Subportfolio_Id,
    subportfolio_name: Option<Subportfolio_Name>,
    subportfolio_description: Option<SerializationLayer<Option<Subportfolio_Description>>>,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::Subportfolio_DoesNotExist,
        CommonPrecedent::Subportfolio_AlreadyExists,
    }
);

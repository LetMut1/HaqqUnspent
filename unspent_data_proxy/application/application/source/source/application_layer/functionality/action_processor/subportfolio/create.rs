#[cfg(not(feature = "not_authorized_user"))]
use crate::infrastructure_layer::data::control_type::AccessToken;
pub use crate::infrastructure_layer::data::control_type::Subportfolio___Create;
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
                Subportfolio_CreatedAt,
                Subportfolio_Description,
                Subportfolio_Id,
                Subportfolio_Name,
                Subportfolio_UpdatedAt,
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
                by::{
                    By2,
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
use serde::{
    Deserialize,
    Serialize,
};

impl ActionProcessor<Subportfolio___Create> {
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

        let is_valid_subportfolio_name = match Validator::<Subportfolio_Name>::is_valid(incoming_.subportfolio_name.as_str()) {
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

        if let Some(ref subportfolio_description) = incoming_.subportfolio_description {
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

        let subportfolio_quantity = match ClickhouseRepository::<Subportfolio>::get_count(
            &clickhouse_client,
            By2 {
                user_id: user_.id,
            },
        )
        .await
        {
            Ok(subportfolio_quantity_) => subportfolio_quantity_,
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

        if (subportfolio_quantity + 1) > Subportfolio::MAXIMUM_QUANTITY_PER_USER {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::Subportfolio_MaximumQuantityPerUser),
                },
            );
        }

        let is_exist = match ClickhouseRepository::<Subportfolio>::is_exist_2(
            &clickhouse_client,
            &By7 {
                user_id: user_.id,
                subportfolio_name: incoming_.subportfolio_name.as_str(),
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

        let subportfolio_id = Creator::<Subportfolio_Id>::create();

        let subportfolio_created_at = match Resolver::<UTCDateTime>::get_now_() {
            Ok(subportfolio_created_at_) => subportfolio_created_at_,
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
            id: subportfolio_id,
            name: Subportfolio_Name(incoming_.subportfolio_name),
            description: incoming_.subportfolio_description,
            created_at: Subportfolio_CreatedAt(subportfolio_created_at),
            updated_at: Subportfolio_UpdatedAt(subportfolio_created_at),
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
                subject: UnifiedReport::filled(
                    Outcoming {
                        subportfolio_id: subportfolio.id,
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
    subportfolio_name: String,
    subportfolio_description: Option<Subportfolio_Description>,
}

#[cfg(not(feature = "not_authorized_user"))]
#[derive(Deserialize)]
pub struct Incoming {
    access_token: AccessToken,
    subportfolio_name: String,
    subportfolio_description: Option<Subportfolio_Description>,
}

#[derive(Serialize)]
pub struct Outcoming {
    subportfolio_id: Subportfolio_Id,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::Subportfolio_MaximumQuantityPerUser,
        CommonPrecedent::Subportfolio_AlreadyExists,
    }
);

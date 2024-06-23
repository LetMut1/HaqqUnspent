pub use crate::infrastructure_layer::data::control_type::SubportfolioTrackableWallet___GetAll;
use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::data::entity::subportfolio_trackable_wallet::SubportfolioTrackableWallet,
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            control_type::ServerAccessToken,
            error::Error,
            invalid_argument::InvalidArgument,
            invalid_argument_result::InvalidArgumentResult,
            void::Void,
        },
        functionality::{
            repository::clickhouse::{
                by::By11,
                subportfolio_trackable_wallet::SubportfolioTrackableWalletAggregated,
                ClickhouseRepository,
            },
            service::validator::Validator,
        },
    },
};
use clickhouse::Client;
use serde::{
    Deserialize,
    Serialize,
};

impl ActionProcessor<SubportfolioTrackableWallet___GetAll> {
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

        if !Validator::<ServerAccessToken>::is_valid(&incoming_.server_access_token) {
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

        let subportfolio_trackable_wallet_aggregated_registry = match ClickhouseRepository::<SubportfolioTrackableWallet>::get(
            &clickhouse_client,
            By11 {
                user_id: incoming_.user_id,
            },
            incoming_.limit,
        )
        .await
        {
            Ok(subportfolio_trackable_wallet_aggregated_registry_) => subportfolio_trackable_wallet_aggregated_registry_,
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
                        subportfolio_trackable_wallet_aggregated_registry,
                    },
                ),
            },
        );
    }
}

#[derive(Deserialize)]
pub struct Incoming {
    server_access_token: ServerAccessToken,
    user_id: Option<i32>,
    limit: i16,
}

#[derive(Serialize)]
pub struct Outcoming {
    subportfolio_trackable_wallet_aggregated_registry: Vec<SubportfolioTrackableWalletAggregated>,
}

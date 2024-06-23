pub use crate::infrastructure_layer::data::control_type::BalanceSnapshot___Create;
use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::balance_snapshot::{
            BalanceSnapshot,
            BalanceSnapshot_TotalAmount,
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
            void::Void,
        },
        functionality::{
            repository::clickhouse::ClickhouseRepository,
            service::validator::{
                server_access_token::ServerAccessToken,
                Validator as Validator_,
            },
        },
    },
};
use clickhouse::Client;
use serde::Deserialize;

impl ActionProcessor<BalanceSnapshot___Create> {
    pub async fn process(incoming: Option<Incoming>, clickhouse_client: Client) -> Result<InvalidArgumentResult<UnifiedReport<Void, Void>>, Auditor<Error>> {
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

        if !Validator_::<ServerAccessToken>::is_valid(&incoming_.server_access_token) {
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

        let mut balance_snapshot_registry: Vec<BalanceSnapshot> = vec![];

        'a: for balance_snapshot in incoming_.balance_snapshot_registry.into_iter() {
            let is_valid_balance_snapshot_total_amount = match Validator::<BalanceSnapshot_TotalAmount>::is_valid(&balance_snapshot.total_amount) {
                Ok(is_valid_balance_snapshot_total_amount_) => is_valid_balance_snapshot_total_amount_,
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

            if !is_valid_balance_snapshot_total_amount {
                continue 'a;
            }

            balance_snapshot_registry.push(balance_snapshot);
        }

        if !balance_snapshot_registry.is_empty() {
            if let Err(mut error_auditor) = ClickhouseRepository::<BalanceSnapshot>::create(
                &clickhouse_client,
                balance_snapshot_registry.as_slice(),
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
        }

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::empty(),
            },
        );
    }
}

#[derive(Deserialize)]
pub struct Incoming {
    server_access_token: ServerAccessToken,
    balance_snapshot_registry: Vec<BalanceSnapshot>,
}

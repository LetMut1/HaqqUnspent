pub use crate::infrastructure_layer::data::control_type::Task___HealthCheck;
use crate::{
    application_layer::functionality::action_processor::ActionProcessor,
    domain_layer::{
        data::entity::task::{
            generate_aggregated_balance_snapshot::GenerateAggregatedBalanceSnapshot,
            generate_asset_snapshot::GenerateAssetSnapshot,
            generate_base_balance_snapshot::GenerateBaseBalanceSnapshot,
            update_assets::UpdateAssets,
            update_assets_for_subportfolio_trackable_wallet::UpdateAssetsForSubportfolioTrackableWallet,
            NamedTask,
            Task,
        },
        functionality::service::executor::Executor,
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            environment_configuration::EnvironmentConfiguration,
            error::Error,
            invalid_argument::InvalidArgument,
            invalid_argument_result::InvalidArgumentResult,
        },
        functionality::service::validator::{
            server_access_token::ServerAccessToken,
            Validator as Validator_,
        },
    },
};
use serde::{
    Deserialize,
    Serialize,
};
use std::sync::Arc;

impl ActionProcessor<Task___HealthCheck> {
    const MAXIMUM_UPDATE_ASSETS_QUANTITY_OF_CONSECUTIVE_TERMINATIONS_WITH_ERRORS: usize = 2;
    const MAXIMUM_GENERATE_ASSET_SNAPSHOT_QUANTITY_OF_CONSECUTIVE_TERMINATIONS_WITH_ERRORS: usize = 2;
    const MAXIMUM_GENERATE_AGGREGATED_BALANCE_SNAPSHOT_QUANTITY_OF_CONSECUTIVE_TERMINATIONS_WITH_ERRORS: usize = 2;
    const MAXIMUM_GENERATE_BASE_BALANCE_SNAPSHOT_QUANTITY_OF_CONSECUTIVE_TERMINATIONS_WITH_ERRORS: usize = 2;
    const MAXIMUM_UPDATE_ASSETS_FOR_SUBPORTFOLIO_TRACKABLE_WALLET_QUANTITY_OF_CONSECUTIVE_TERMINATIONS_WITH_ERRORS: usize = 2;

    pub async fn process(incoming: Option<Incoming>, environment_configuration: Arc<EnvironmentConfiguration>) -> Result<InvalidArgumentResult<Option<Outcoming>>, Auditor<Error>> {
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

        if !Validator_::<ServerAccessToken>::is_valid(
            environment_configuration.as_ref(),
            &incoming_.server_access_token,
        ) {
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

        let task_name = incoming_.task_name.as_str();

        let is_task_executed_by_shedule = if task_name == UpdateAssets::get_name() {
            let is_task_executed_by_shedule_ = if Executor::<Task<UpdateAssets>>::get_quantity_of_consecutive_terminations_with_errors().await
                <= Self::MAXIMUM_UPDATE_ASSETS_QUANTITY_OF_CONSECUTIVE_TERMINATIONS_WITH_ERRORS
            {
                true
            } else {
                false
            };

            is_task_executed_by_shedule_
        } else {
            let is_task_executed_by_shedule_ = if task_name == GenerateAssetSnapshot::get_name() {
                let is_task_executed_by_shedule__ = if Executor::<Task<GenerateAssetSnapshot>>::get_quantity_of_consecutive_terminations_with_errors().await
                    <= Self::MAXIMUM_GENERATE_ASSET_SNAPSHOT_QUANTITY_OF_CONSECUTIVE_TERMINATIONS_WITH_ERRORS
                {
                    true
                } else {
                    false
                };

                is_task_executed_by_shedule__
            } else {
                let is_task_executed_by_shedule__ = if task_name == GenerateAggregatedBalanceSnapshot::get_name() {
                    let is_task_executed_by_shedule___ = if Executor::<Task<GenerateAggregatedBalanceSnapshot>>::get_quantity_of_consecutive_terminations_with_errors().await
                        <= Self::MAXIMUM_GENERATE_AGGREGATED_BALANCE_SNAPSHOT_QUANTITY_OF_CONSECUTIVE_TERMINATIONS_WITH_ERRORS
                    {
                        true
                    } else {
                        false
                    };

                    is_task_executed_by_shedule___
                } else {
                    let is_task_executed_by_shedule___ = if task_name == GenerateBaseBalanceSnapshot::get_name() {
                        let is_task_executed_by_shedule____ = if Executor::<Task<GenerateBaseBalanceSnapshot>>::get_quantity_of_consecutive_terminations_with_errors().await
                            <= Self::MAXIMUM_GENERATE_BASE_BALANCE_SNAPSHOT_QUANTITY_OF_CONSECUTIVE_TERMINATIONS_WITH_ERRORS
                        {
                            true
                        } else {
                            false
                        };

                        is_task_executed_by_shedule____
                    } else {
                        let is_task_executed_by_shedule____ = if task_name == UpdateAssetsForSubportfolioTrackableWallet::get_name() {
                            let is_task_executed_by_shedule_____ =
                                if Executor::<Task<UpdateAssetsForSubportfolioTrackableWallet>>::get_quantity_of_consecutive_terminations_with_errors().await
                                    <= Self::MAXIMUM_UPDATE_ASSETS_FOR_SUBPORTFOLIO_TRACKABLE_WALLET_QUANTITY_OF_CONSECUTIVE_TERMINATIONS_WITH_ERRORS
                                {
                                    true
                                } else {
                                    false
                                };

                            is_task_executed_by_shedule_____
                        } else {
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
                        };

                        is_task_executed_by_shedule____
                    };

                    is_task_executed_by_shedule___
                };

                is_task_executed_by_shedule__
            };

            is_task_executed_by_shedule_
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: Some(
                    Outcoming {
                        is_task_executed_by_shedule,
                    },
                ),
            },
        );
    }
}

#[derive(Deserialize)]
pub struct Incoming {
    server_access_token: ServerAccessToken,
    task_name: String,
}

#[derive(Serialize)]
pub struct Outcoming {
    is_task_executed_by_shedule: bool,
}

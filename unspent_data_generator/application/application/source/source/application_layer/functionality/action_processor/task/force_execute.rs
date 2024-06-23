pub use crate::infrastructure_layer::data::control_type::Task___ForceExecute;
use crate::{
    application_layer::functionality::action_processor::ActionProcessor,
    domain_layer::{
        data::entity::task::{
            generate_aggregated_balance_snapshot::GenerateAggregatedBalanceSnapshot,
            generate_asset_snapshot::GenerateAssetSnapshot,
            generate_base_balance_snapshot::GenerateBaseBalanceSnapshot,
            update_assets___generate_asset_snapshot::UpdateAssets__GenerateAssetSnapshot,
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
            control_type::Common,
            environment_configuration::EnvironmentConfiguration,
            error::Error,
            invalid_argument::InvalidArgument,
            invalid_argument_result::InvalidArgumentResult,
        },
        functionality::service::{
            logger::Logger,
            spawner::{
                tokio_non_blocking_task::TokioNonBlockingTask,
                Spawner,
            },
            validator::{
                server_access_token::ServerAccessToken,
                Validator as Validator_,
            },
        },
        macro_rules::{
            task_executing_by_force_execute_pattern,
            task_started_to_execute_pattern,
        },
    },
};
use serde::{
    Deserialize,
    Serialize,
};
use std::sync::Arc;

impl ActionProcessor<Task___ForceExecute> {
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

        let update_assets___generate_asset_snapshot_task_name = UpdateAssets__GenerateAssetSnapshot::get_name();

        let generate_asset_snapshot_task_name = GenerateAssetSnapshot::get_name();

        let generate_aggregated_balance_snapshot_task_name = GenerateAggregatedBalanceSnapshot::get_name();

        let generate_base_balance_snapshot_task_name = GenerateBaseBalanceSnapshot::get_name();

        let update_assets_for_subportfolio_trackable_wallet_task_name = UpdateAssetsForSubportfolioTrackableWallet::get_name();

        let task_name = incoming_.task_name.as_str();

        let message = if task_name == update_assets___generate_asset_snapshot_task_name {
            let update_assets___generate_asset_snapshot_task_name_ = update_assets___generate_asset_snapshot_task_name;

            let future = async move {
                let result = match Executor::<Task<UpdateAssets__GenerateAssetSnapshot>>::execute(environment_configuration).await {
                    Ok(_) => {
                        let message = format!(
                            task_executing_by_force_execute_pattern!(),
                            update_assets___generate_asset_snapshot_task_name_,
                        );

                        Logger::<Common>::log_info(message.as_str());

                        Ok(())
                    }
                    Err(mut error_auditor) => {
                        error_auditor.add_backtrace_part(
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        );

                        Err(error_auditor)
                    }
                };

                result
            };

            Spawner::<TokioNonBlockingTask>::spawn_into_background(future);

            format!(
                task_started_to_execute_pattern!(),
                update_assets___generate_asset_snapshot_task_name,
            )
        } else {
            let message_ = if task_name == generate_asset_snapshot_task_name {
                let generate_asset_snapshot_task_name_ = generate_asset_snapshot_task_name;

                let future = async move {
                    let result = match Executor::<Task<GenerateAssetSnapshot>>::execute(environment_configuration).await {
                        Ok(_) => {
                            let message = format!(
                                task_executing_by_force_execute_pattern!(),
                                generate_asset_snapshot_task_name_,
                            );

                            Logger::<Common>::log_info(message.as_str());

                            Ok(())
                        }
                        Err(mut error_auditor) => {
                            error_auditor.add_backtrace_part(
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                ),
                            );

                            Err(error_auditor)
                        }
                    };

                    result
                };

                Spawner::<TokioNonBlockingTask>::spawn_into_background(future);

                format!(
                    task_started_to_execute_pattern!(),
                    generate_asset_snapshot_task_name,
                )
            } else {
                let message__ = if task_name == generate_aggregated_balance_snapshot_task_name {
                    let generate_aggregated_balance_snapshot_task_name_ = generate_aggregated_balance_snapshot_task_name;

                    let future = async move {
                        let result = match Executor::<Task<GenerateAggregatedBalanceSnapshot>>::execute(environment_configuration).await {
                            Ok(_) => {
                                let message = format!(
                                    task_executing_by_force_execute_pattern!(),
                                    generate_aggregated_balance_snapshot_task_name_,
                                );

                                Logger::<Common>::log_info(message.as_str());

                                Ok(())
                            }
                            Err(mut error_auditor) => {
                                error_auditor.add_backtrace_part(
                                    BacktracePart::new(
                                        line!(),
                                        file!(),
                                    ),
                                );

                                Err(error_auditor)
                            }
                        };

                        result
                    };

                    Spawner::<TokioNonBlockingTask>::spawn_into_background(future);

                    format!(
                        task_started_to_execute_pattern!(),
                        generate_aggregated_balance_snapshot_task_name,
                    )
                } else {
                    let message___ = if task_name == generate_base_balance_snapshot_task_name {
                        let generate_base_balance_snapshot_task_name_ = generate_base_balance_snapshot_task_name;

                        let future = async move {
                            let result = match Executor::<Task<GenerateBaseBalanceSnapshot>>::execute(environment_configuration).await {
                                Ok(_) => {
                                    let message = format!(
                                        task_executing_by_force_execute_pattern!(),
                                        generate_base_balance_snapshot_task_name_,
                                    );

                                    Logger::<Common>::log_info(message.as_str());

                                    Ok(())
                                }
                                Err(mut error_auditor) => {
                                    error_auditor.add_backtrace_part(
                                        BacktracePart::new(
                                            line!(),
                                            file!(),
                                        ),
                                    );

                                    Err(error_auditor)
                                }
                            };

                            result
                        };

                        Spawner::<TokioNonBlockingTask>::spawn_into_background(future);

                        format!(
                            task_started_to_execute_pattern!(),
                            generate_base_balance_snapshot_task_name,
                        )
                    } else {
                        let message____ = if task_name == update_assets_for_subportfolio_trackable_wallet_task_name {
                            let update_assets_for_subportfolio_trackable_wallet_task_name_ = update_assets_for_subportfolio_trackable_wallet_task_name;

                            let future = async move {
                                let result = match Executor::<Task<GenerateBaseBalanceSnapshot>>::execute(environment_configuration).await {
                                    Ok(_) => {
                                        let message = format!(
                                            task_executing_by_force_execute_pattern!(),
                                            update_assets_for_subportfolio_trackable_wallet_task_name_,
                                        );

                                        Logger::<Common>::log_info(message.as_str());

                                        Ok(())
                                    }
                                    Err(mut error_auditor) => {
                                        error_auditor.add_backtrace_part(
                                            BacktracePart::new(
                                                line!(),
                                                file!(),
                                            ),
                                        );

                                        Err(error_auditor)
                                    }
                                };

                                result
                            };

                            Spawner::<TokioNonBlockingTask>::spawn_into_background(future);

                            format!(
                                task_started_to_execute_pattern!(),
                                update_assets_for_subportfolio_trackable_wallet_task_name,
                            )
                        } else {
                            format!(
                                "The task does not exist. Existing tasks: '{}', '{}', '{}', '{}', '{}'.",
                                update_assets___generate_asset_snapshot_task_name, generate_asset_snapshot_task_name, generate_aggregated_balance_snapshot_task_name, generate_base_balance_snapshot_task_name, update_assets_for_subportfolio_trackable_wallet_task_name,
                            )
                        };

                        message____
                    };

                    message___
                };

                message__
            };

            message_
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: Some(
                    Outcoming {
                        message,
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
    message: String,
}

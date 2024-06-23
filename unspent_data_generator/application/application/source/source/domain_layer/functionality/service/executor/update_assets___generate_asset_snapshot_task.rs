use super::Executor;
use crate::{
    domain_layer::data::entity::task::{
        generate_asset_snapshot::GenerateAssetSnapshot,
        update_assets::UpdateAssets,
        update_assets___generate_asset_snapshot::UpdateAssets__GenerateAssetSnapshot,
        Task,
    },
    infrastructure_layer::data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        environment_configuration::EnvironmentConfiguration,
        error::Error,
    },
};
use std::sync::Arc;
use tokio::time::{
    sleep,
    Duration,
};

impl Executor<Task<UpdateAssets__GenerateAssetSnapshot>> {
    const TASK_REPETITIONS_QUANTITY: u8 = 2;

    const TASK_SLEEPING_TIME_AFTER_ERROR_IN_SECONDS: u64 = 10;

    pub async fn execute(environment_configuration: Arc<EnvironmentConfiguration>) -> Result<(), Auditor<Error>> {
        let mut is_needed_to_execute_update_assets_task = true;

        let mut is_needed_to_execute_generate_assets_snapshot_task = true;

        let mut counter: u8 = 0;

        'a: loop {
            if let Err(mut error_auditor) = Self::execute_(
                environment_configuration.clone(),
                &mut is_needed_to_execute_update_assets_task,
                &mut is_needed_to_execute_generate_assets_snapshot_task,
            )
            .await
            {
                if counter < Self::TASK_REPETITIONS_QUANTITY {
                    counter += 1;

                    sleep(Duration::from_secs(Self::TASK_SLEEPING_TIME_AFTER_ERROR_IN_SECONDS)).await;

                    continue 'a;
                } else {
                    error_auditor.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    );

                    return Err(error_auditor);
                }
            }

            break 'a;
        }

        return Ok(());
    }

    async fn execute_<'a>(
        environment_configuration: Arc<EnvironmentConfiguration>,
        is_needed_to_execute_update_assets_task: &'a mut bool,
        is_needed_to_execute_generate_assets_snapshot_task: &'a mut bool,
    ) -> Result<(), Auditor<Error>> {
        if *is_needed_to_execute_update_assets_task {
            if let Err(mut error_auditor) = Executor::<Task<UpdateAssets>>::execute(environment_configuration.clone()).await {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }

            *is_needed_to_execute_update_assets_task = false;
        }

        if *is_needed_to_execute_generate_assets_snapshot_task {
            if let Err(mut error_auditor) = Executor::<Task<GenerateAssetSnapshot>>::execute(environment_configuration.clone()).await {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }

            *is_needed_to_execute_generate_assets_snapshot_task = false;
        }

        return Ok(());
    }
}

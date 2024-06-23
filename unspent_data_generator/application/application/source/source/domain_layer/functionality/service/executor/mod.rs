pub mod generate_aggregated_balance_snapshot_task;
pub mod generate_asset_snapshot_task;
pub mod generate_base_balance_snapshot_task;
pub mod update_assets___generate_asset_snapshot_task;
pub mod update_assets_for_subportfolio_trackable_wallet;
pub mod update_assets_task;

use std::marker::PhantomData;

pub struct Executor<S> {
    _subject: PhantomData<S>,
}

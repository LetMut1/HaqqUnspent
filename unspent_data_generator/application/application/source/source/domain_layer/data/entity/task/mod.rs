pub mod generate_aggregated_balance_snapshot;
pub mod generate_asset_snapshot;
pub mod generate_base_balance_snapshot;
pub mod update_assets;
pub mod update_assets___generate_asset_snapshot;
pub mod update_assets_for_subportfolio_trackable_wallet;

use std::marker::PhantomData;

pub struct Task<S> {
    _subject: PhantomData<S>,
}

pub trait NamedTask {
    fn get_name() -> &'static str;
}

pub mod asset_snapshot;
pub mod balance_snapshot;
pub mod base_balance_snapshot;
pub mod cors_preflight_request;
pub mod health_check_1;
pub mod health_check_2;
pub mod route_not_found;
pub mod subportfolio;
pub mod subportfolio_asset;
pub mod subportfolio_base_balance_snapshot;
pub mod subportfolio_link;
pub mod subportfolio_trackable_wallet;

use std::marker::PhantomData;

pub struct Action<S> {
    _subject: PhantomData<S>,
}

pub mod aggregated_balance_snapshot__total_amount;
pub mod asset_snapshot__price_btc;
pub mod asset_snapshot__price_usd;
pub mod base_balance_snapshot__amount;

use std::marker::PhantomData;

pub struct Validator<S> {
    _subject: PhantomData<S>,
}

pub mod asset_snapshot__price_btc;
pub mod asset_snapshot__price_usd;
pub mod balance_snapshot__total_amount;
pub mod base_balance_snapshot__amount;
pub mod subportfolio__description;
pub mod subportfolio__id;
pub mod subportfolio__name;
pub mod subportfolio_link__description;
pub mod subportfolio_link__id;

use std::marker::PhantomData;

pub struct Validator<S> {
    _subject: PhantomData<S>,
}

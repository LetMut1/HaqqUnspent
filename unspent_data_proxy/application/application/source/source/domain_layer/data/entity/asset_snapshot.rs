pub use self::{
    CreatedAt as AssetSnapshot_CreatedAt,
    PriceBtc as AssetSnapshot_PriceBtc,
    PriceUsd as AssetSnapshot_PriceUsd,
};
use super::_remote::Asset_Id;
use clickhouse::{
    self,
    Row,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PriceUsd(pub String);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct PriceBtc(pub String);

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreatedAt(pub u32);

#[derive(Row, Serialize, Deserialize)]
pub struct AssetSnapshot {
    pub asset_id: Asset_Id,
    pub price_usd: PriceUsd,
    pub price_btc: Option<PriceBtc>,
    pub created_at: CreatedAt,
}

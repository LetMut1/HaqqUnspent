pub use self::{
    CreatedAt as AssetSnapshotFor120Days_CreatedAt,
    PriceBtc as AssetSnapshotFor120Days_PriceBtc,
    PriceUsd as AssetSnapshotFor120Days_PriceUsd,
};
use super::_remote::Asset_Id;

pub struct PriceUsd(pub String);

pub struct PriceBtc(pub String);

pub struct CreatedAt(pub u32);

pub struct AssetSnapshotFor120Days {
    pub asset_id: Asset_Id,
    pub price_usd: PriceUsd,
    pub price_btc: Option<PriceBtc>,
    pub created_at: CreatedAt,
}

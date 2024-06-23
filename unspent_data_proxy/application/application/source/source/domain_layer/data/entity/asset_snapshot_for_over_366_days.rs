pub use self::{
    CreatedAt as AssetSnapshotForOver366Days_CreatedAt,
    PriceBtc as AssetSnapshotForOver366Days_PriceBtc,
    PriceUsd as AssetSnapshotForOver366Days_PriceUsd,
};
use super::_remote::Asset_Id;

pub struct PriceUsd(pub String);

pub struct PriceBtc(pub String);

pub struct CreatedAt(pub u32);

pub struct AssetSnapshotForOver366Days {
    pub asset_id: Asset_Id,
    pub price_usd: PriceUsd,
    pub price_btc: Option<PriceBtc>,
    pub created_at: CreatedAt,
}

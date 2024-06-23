pub use self::{
    CreatedAt as AggregatedBalanceSnapshot_CreatedAt,
    TotalAmount as AggregatedBalanceSnapshot_TotalAmount,
};
use super::{
    _remote::User_Id,
    asset::Asset_Id,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TotalAmount(pub String);

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreatedAt(pub u32);

#[derive(Serialize, Deserialize)]
pub struct AggregatedBalanceSnapshot {
    pub user_id: User_Id,
    pub asset_id: Asset_Id,
    pub total_amount: TotalAmount,
    pub created_at: CreatedAt,
}

#[derive(Serialize, Deserialize)]
pub struct AggregatedBalanceSnapshot_1 {
    pub user_id: User_Id,
    pub asset_id: Asset_Id,
    pub total_amount: TotalAmount,
}

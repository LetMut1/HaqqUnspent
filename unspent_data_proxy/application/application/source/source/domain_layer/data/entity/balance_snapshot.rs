pub use self::{
    CreatedAt as BalanceSnapshot_CreatedAt,
    TotalAmount as BalanceSnapshot_TotalAmount,
};
pub use super::_remote::{
    Asset_Id,
    User_Id,
};
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
pub struct TotalAmount(pub String);

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreatedAt(pub u32);

#[derive(Row, Serialize, Deserialize)]
pub struct BalanceSnapshot {
    pub user_id: User_Id,
    pub asset_id: Asset_Id,
    pub total_amount: TotalAmount,
    pub created_at: CreatedAt,
}

pub use self::{
    CreatedAt as BalanceSnapshotFor7Days_CreatedAt,
    TotalAmount as BalanceSnapshotFor7Days_TotalAmount,
};
use super::_remote::{
    Asset_Id,
    User_Id,
};

pub struct TotalAmount(pub String);

pub struct CreatedAt(pub u32);

pub struct BalanceSnapshotFor7Days {
    pub user_id: User_Id,
    pub asset_id: Asset_Id,
    pub total_amount: TotalAmount,
    pub created_at: CreatedAt,
}

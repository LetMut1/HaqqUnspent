pub use self::{
    CreatedAt as BalanceSnapshotForOver366Days_CreatedAt,
    TotalAmount as BalanceSnapshotForOver366Days_TotalAmount,
};
use super::_remote::{
    Asset_Id,
    User_Id,
};

pub struct TotalAmount(pub String);

pub struct CreatedAt(pub u32);

pub struct BalanceSnapshotForOver366Days {
    pub user_id: User_Id,
    pub asset_id: Asset_Id,
    pub total_amount: TotalAmount,
    pub created_at: CreatedAt,
}

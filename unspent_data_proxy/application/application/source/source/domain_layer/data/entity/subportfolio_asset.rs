pub use self::{
    CreatedAt as SubportfolioAsset_CreatedAt,
    UpdatedAt as SubportfolioAsset_UpdatedAt,
};
use super::{
    _remote::{
        Asset_ChainId,
        Asset_Id,
        Asset_Network,
        Exchange_Id,
        Exchange_Name,
        User_Id,
        Wallet_Address,
        Wallet_Id,
        Wallet_Label,
    },
    subportfolio::Subportfolio_Id,
};
use crate::domain_layer::data::common_type::IsDeleted;
use clickhouse::{
    self,
    Row,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreatedAt(pub u32);

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UpdatedAt(pub u32);

pub struct SubportfolioAsset {
    pub user_id: User_Id,
    pub subportfolio_id: Subportfolio_Id,
    pub exchange_id: Exchange_Id,
    pub exchange_name: Exchange_Name,
    pub wallet_id: Wallet_Id,
    pub wallet_address: Wallet_Address,
    pub wallet_label: Wallet_Label,
    pub asset_network: Asset_Network,
    pub asset_chain_id: Asset_ChainId,
    pub asset_id: Asset_Id,
    pub created_at: CreatedAt,
    // Special value for correct work with the database engine.
    pub updated_at: UpdatedAt,
    // Special value for correct work with the database engine.
    pub is_deleted: IsDeleted,
}

impl SubportfolioAsset {
    pub const MAXIMUM_QUANTITY_PER_SUBPORTFOLIO: usize = 500;
}

#[derive(Row, Deserialize)]
pub struct SubportfolioAsset_1 {
    pub exchange_id: Exchange_Id,
    pub wallet_id: Wallet_Id,
    pub asset_network: Asset_Network,
    pub asset_chain_id: Asset_ChainId,
    pub asset_id: Asset_Id,
}

#[derive(Row, Serialize, Deserialize)]
pub struct SubportfolioAsset_2 {
    pub exchange_id: Exchange_Id,
    pub exchange_name: Exchange_Name,
    pub wallet_id: Wallet_Id,
    pub wallet_address: Wallet_Address,
    pub wallet_label: Wallet_Label,
    pub asset_network: Asset_Network,
    pub asset_chain_id: Asset_ChainId,
    pub asset_id: Asset_Id,
    pub created_at: CreatedAt,
}

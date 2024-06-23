pub use self::{
    Amount as BaseBalanceSnapshot_Amount,
    CreatedAt as BaseBalanceSnapshot_CreatedAt,
};
use super::_remote::{
    Asset_ChainId,
    Asset_Id,
    Asset_Network,
    Exchange_Id,
    Exchange_Name,
    User_Id,
    Wallet_Address,
    Wallet_Id,
    Wallet_Label,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Amount(pub String);

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreatedAt(pub u32);

pub struct BaseBalanceSnapshot {
    pub user_id: User_Id,
    pub exchange_id: Exchange_Id,
    pub exchange_name: Exchange_Name,
    pub wallet_id: Wallet_Id,
    pub wallet_address: Wallet_Address,
    pub wallet_label: Wallet_Label,
    pub asset_network: Asset_Network,
    pub asset_chain_id: Asset_ChainId,
    pub asset_id: Asset_Id,
    pub amount: Amount,
    pub created_at: CreatedAt,
}

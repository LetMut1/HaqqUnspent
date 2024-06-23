pub use self::{
    Amount as BaseBalanceSnapshot_Amount,
    CreatedAt as BaseBalanceSnapshot_CreatedAt,
};
use super::{
    _remote::{
        Asset_ChainId,
        Asset_Network,
        Exchange_Id,
        Exchange_Name,
        User_Id,
        Wallet_Address,
        Wallet_Id,
        Wallet_Label,
    },
    asset::Asset_Id,
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

#[derive(Serialize, Deserialize)]
pub struct BaseBalanceSnapshot {
    pub user_id: User_Id,
    pub exchange_id: Option<Exchange_Id>,
    pub exchange_name: Option<Exchange_Name>,
    pub wallet_id: Option<Wallet_Id>,
    pub wallet_address: Option<Wallet_Address>,
    pub wallet_label: Option<Wallet_Label>,
    pub asset_network: Option<Asset_Network>,
    pub asset_chain_id: Option<Asset_ChainId>,
    pub asset_id: Asset_Id,
    pub amount: Amount,
    pub created_at: CreatedAt,
}

#[derive(Serialize, Deserialize)]
pub struct BaseBalanceSnapshot_1 {
    pub user_id: User_Id,
    pub exchange_id: Option<Exchange_Id>,
    pub exchange_name: Option<Exchange_Name>,
    pub wallet_id: Option<Wallet_Id>,
    pub wallet_address: Option<Wallet_Address>,
    pub wallet_label: Option<Wallet_Label>,
    pub asset_network: Option<Asset_Network>,
    pub asset_chain_id: Option<Asset_ChainId>,
    pub asset_id: Asset_Id,
    pub amount: Amount,
}

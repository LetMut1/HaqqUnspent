pub use self::{
    CreatedAt as SubportfolioTrackableWallet_CreatedAt,
    UpdatedAt as SubportfolioTrackableWallet_UpdatedAt,
};
use super::{
    _remote::{
        User_Id,
        Wallet_Id,
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
use std::marker::PhantomData;

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreatedAt(pub u32);

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UpdatedAt(pub u32);

pub struct SubportfolioTrackableWallet {
    pub wallet_id: i32,
    _wallet_id: PhantomData<Wallet_Id>,

    pub user_id: i32,
    _user_id: PhantomData<User_Id>,

    pub subportfolio_id: String,
    _subportfolio_id: PhantomData<Subportfolio_Id>,

    pub created_at: u32,
    _created_at: PhantomData<CreatedAt>,

    // Special value for correct work with the database engine.
    pub updated_at: u32,
    _updated_at: PhantomData<UpdatedAt>,

    // Special value for correct work with the database engine.
    pub is_deleted: u8,
    _is_deleted: PhantomData<IsDeleted>,
}

impl SubportfolioTrackableWallet {
    pub const MAXIMUM_QUANTITY_PER_USER_AND_SUBPORTFOLIO: usize = 25;

    pub fn new(wallet_id: i32, user_id: i32, subportfolio_id: String, created_at: u32, updated_at: u32, is_deleted: u8) -> Self {
        return Self {
            wallet_id,
            _wallet_id: PhantomData,
            user_id,
            _user_id: PhantomData,
            subportfolio_id,
            _subportfolio_id: PhantomData,
            created_at,
            _created_at: PhantomData,
            updated_at,
            _updated_at: PhantomData,
            is_deleted,
            _is_deleted: PhantomData,
        };
    }
}

#[derive(Row, Deserialize)]
pub struct SubportfolioTrackableWallet_1 {
    pub wallet_id: i32,
}

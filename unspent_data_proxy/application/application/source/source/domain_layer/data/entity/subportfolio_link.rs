pub use self::{
    CreatedAt as SubportfolioLink_CreatedAt,
    Description as SubportfolioLink_Description,
    Id as SubportfolioLink_Id,
    UpdatedAt as SubportfolioLink_UpdatedAt,
};
use super::{
    _remote::User_Id,
    subportfolio::Subportfolio_Id,
};
pub use crate::domain_layer::data::common_type::IsDeleted;
use clickhouse::{
    self,
    Row,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::marker::PhantomData;

pub struct Id;

pub struct IsActive;

pub struct Description;

impl Description {
    pub const MAXIMUM_BYTES_QUANTITY: usize = 1024 * 10;
}

pub struct CreatedAt;

pub struct UpdatedAt;

pub struct SubportfolioLink {
    pub id: String,
    _id: PhantomData<Id>,

    pub user_id: i32,
    _user_id: PhantomData<User_Id>,

    pub subportfolio_id: String,
    _subportfolio_id: PhantomData<Subportfolio_Id>,

    pub is_active: bool,
    _is_active: PhantomData<IsActive>,

    pub description: Option<String>,
    _description: PhantomData<Description>,

    pub created_at: u32,
    _created_at: PhantomData<CreatedAt>,

    // Special value for correct work with the database engine.
    pub updated_at: u32,
    _updated_at: PhantomData<UpdatedAt>,

    // Special value for correct work with the database engine.
    pub is_deleted: u8,
    _is_deleted: PhantomData<IsDeleted>,
}

impl SubportfolioLink {
    pub const MAXIMUM_QUANTITY_PER_USER_AND_SUBPORTFOLIO: u64 = 100;

    pub fn new(id: String, user_id: i32, subportfolio_id: String, is_active: bool, description: Option<String>, created_at: u32, updated_at: u32, is_deleted: u8) -> Self {
        return Self {
            id,
            _id: PhantomData,
            user_id,
            _user_id: PhantomData,
            subportfolio_id,
            _subportfolio_id: PhantomData,
            is_active,
            _is_active: PhantomData,
            description,
            _description: PhantomData,
            created_at,
            _created_at: PhantomData,
            updated_at,
            _updated_at: PhantomData,
            is_deleted,
            _is_deleted: PhantomData,
        };
    }
}

#[derive(Serialize)]
pub struct SubportfolioLink_1 {
    pub id: String,
    pub is_active: bool,
    pub description: Option<String>,
    pub created_at: u32,
}

pub struct SubportfolioLink_2 {
    pub subportfolio_id: String,
    pub is_active: bool,
    pub description: Option<String>,
    pub created_at: u32,
}

#[derive(Row, Serialize, Deserialize)]
pub struct SubportfolioLink_3 {
    pub user_id: i32,
    pub subportfolio_id: String,
    pub is_active: bool,
}

#[derive(Row, Serialize, Deserialize)]
pub struct SubportfolioLink_4 {
    pub is_active: bool,
}

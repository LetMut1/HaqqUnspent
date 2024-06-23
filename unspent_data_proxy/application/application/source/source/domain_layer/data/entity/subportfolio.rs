pub use self::{
    CreatedAt as Subportfolio_CreatedAt,
    Description as Subportfolio_Description,
    Id as Subportfolio_Id,
    Name as Subportfolio_Name,
    UpdatedAt as Subportfolio_UpdatedAt,
};
use super::_remote::User_Id;
pub use crate::domain_layer::data::common_type::IsDeleted;
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
pub struct Id(pub String);

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Name(pub String);

impl Name {
    pub const MAXIMUM_BYTES_QUANTITY: usize = 128;
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Description(pub String);

impl Description {
    pub const MAXIMUM_BYTES_QUANTITY: usize = 1024 * 10;
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreatedAt(pub u32);

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UpdatedAt(pub u32);

pub struct Subportfolio {
    pub user_id: User_Id,
    pub id: Id,
    pub name: Name,
    pub description: Option<Description>,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
    // Special value for correct work with the database engine.
    pub is_deleted: IsDeleted,
}

impl Subportfolio {
    pub const MAXIMUM_QUANTITY_PER_USER: u64 = 10;
}

#[derive(Row, Serialize, Deserialize)]
pub struct Subportfolio_1 {
    pub id: Id,
    pub name: Name,
    pub description: Option<Description>,
}

#[derive(Row, Serialize, Deserialize)]
pub struct Subportfolio_2 {
    pub name: Name,
    pub description: Option<Description>,
    pub created_at: CreatedAt,
}

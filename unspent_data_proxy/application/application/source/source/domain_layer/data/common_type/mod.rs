use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IsDeleted(u8);

impl IsDeleted {
    pub fn create_deleted() -> Self {
        return Self(1);
    }

    pub fn create_not_deleted() -> Self {
        return Self(0);
    }

    pub fn get<'a>(&'a self) -> u8 {
        return self.0;
    }
}

use crate::infrastructure_layer::{
    data::control_type::DefaultValue,
    functionality::service::resolver::Resolver,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct User_Id(pub i32);

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Asset_Id(pub String);

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Asset_Network(pub String);

impl Default for Asset_Network {
    fn default() -> Self {
        return Self(Resolver::<DefaultValue>::STRING_DEFAULT_VALUE);
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct Asset_ChainId(pub i32);

impl Default for Asset_ChainId {
    fn default() -> Self {
        return Self(Resolver::<DefaultValue>::I32_DEFAULT_VALUE);
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Wallet_Id(pub i32);

impl Default for Wallet_Id {
    fn default() -> Self {
        return Self(Resolver::<DefaultValue>::I32_DEFAULT_VALUE);
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct Wallet_Label(pub String);

impl Default for Wallet_Label {
    fn default() -> Self {
        return Self(Resolver::<DefaultValue>::STRING_DEFAULT_VALUE);
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct Wallet_Address(pub String);

impl Default for Wallet_Address {
    fn default() -> Self {
        return Self(Resolver::<DefaultValue>::STRING_DEFAULT_VALUE);
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Exchange_Id(pub String);

impl Default for Exchange_Id {
    fn default() -> Self {
        return Self(Resolver::<DefaultValue>::STRING_DEFAULT_VALUE);
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct Exchange_Name(pub String);

impl Default for Exchange_Name {
    fn default() -> Self {
        return Self(Resolver::<DefaultValue>::STRING_DEFAULT_VALUE);
    }
}

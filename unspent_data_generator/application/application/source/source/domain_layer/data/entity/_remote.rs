use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct User_Id(pub i32);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Asset_Id(pub String);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Asset_Network(pub String);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Asset_ChainId(pub i32);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Wallet_Id(pub i32);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Wallet_Label(pub String);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Wallet_Address(pub String);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exchange_Id(pub String);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exchange_Name(pub String);

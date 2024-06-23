use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct SerializationLayer<T> {
    pub data: T,
}

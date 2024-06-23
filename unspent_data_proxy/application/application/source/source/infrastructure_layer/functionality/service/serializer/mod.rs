pub mod json;

use crate::infrastructure_layer::data::{
    auditor::Auditor,
    error::Error,
};
use serde::{
    Deserialize,
    Serialize as SerdeSerialize,
};
use std::marker::PhantomData;

pub struct Serializer<T> {
    _format: PhantomData<T>,
}

pub trait Serialize {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, Auditor<Error>>
    where
        T: SerdeSerialize;

    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, Auditor<Error>>
    where
        T: Deserialize<'a>;
}

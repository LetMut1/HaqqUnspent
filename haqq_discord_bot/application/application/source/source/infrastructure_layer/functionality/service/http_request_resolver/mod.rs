pub mod cosmos;
pub mod evm;

use std::marker::PhantomData;

pub struct HttpRequestResolver<S> {
    _subject: PhantomData<S>,
}

pub mod clickhouse_client;
pub mod response;

use std::marker::PhantomData;

pub struct Creator<S> {
    _subject: PhantomData<S>,
}

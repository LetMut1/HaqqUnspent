pub mod postgresql_connection_pool_no_tls;
pub mod response;

use std::marker::PhantomData;

pub struct Creator<S> {
    _subject: PhantomData<S>,
}

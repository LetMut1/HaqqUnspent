pub mod http_body_data;
pub mod void;

use std::marker::PhantomData;

pub struct Extractor<S> {
    _subject: PhantomData<S>,
}

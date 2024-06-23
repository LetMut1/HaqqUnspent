pub mod access_token;
pub mod default_value;
pub mod utc_date_time;

use std::marker::PhantomData;

pub struct Resolver<S> {
    _subject: PhantomData<S>,
}

impl<S> Resolver<S> {
    pub fn new() -> Self {
        return Self {
            _subject: PhantomData,
        };
    }
}

pub mod environment_configuration;
pub mod utc_date_time;

use std::marker::PhantomData;

pub struct Resolver<S> {
    _subject: PhantomData<S>,
}

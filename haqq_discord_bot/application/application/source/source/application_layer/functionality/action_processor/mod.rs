pub mod health_check;
pub mod route_not_found;

use std::marker::PhantomData;

pub struct ActionProcessor<S> {
    _subject: PhantomData<S>,
}

pub mod health_check;
pub mod route_not_found;
pub mod task;

use std::marker::PhantomData;

pub struct ActionProcessor<S> {
    _subject: PhantomData<S>,
}

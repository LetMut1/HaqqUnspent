pub mod generalized_action;

use std::marker::PhantomData;

pub struct Processor<S> {
    _subject: PhantomData<S>,
}

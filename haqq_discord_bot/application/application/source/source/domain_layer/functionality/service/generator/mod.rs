pub mod raffle__seed;

use std::marker::PhantomData;

pub struct Generator<S> {
    _subject: PhantomData<S>,
}

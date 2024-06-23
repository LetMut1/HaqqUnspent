pub mod snapshot_range;

use std::marker::PhantomData;

pub struct Resolver<S> {
    _subject: PhantomData<S>,
}

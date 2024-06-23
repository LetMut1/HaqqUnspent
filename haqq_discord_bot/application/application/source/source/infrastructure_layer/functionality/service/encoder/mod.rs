pub mod bech32;

use std::marker::PhantomData;

pub struct Encoder<S> {
    _subject: PhantomData<S>,
}

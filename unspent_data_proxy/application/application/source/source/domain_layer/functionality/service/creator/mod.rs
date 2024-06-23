pub mod subportfolio__id;
pub mod subportfolio__name;
pub mod subportfolio_link__id;

use std::marker::PhantomData;

pub struct Creator<S> {
    _subject: PhantomData<S>,
}

impl<S> Creator<S> {
    const STRING_MINIMUM_LENGTH_VALUE: String = String::new();
}

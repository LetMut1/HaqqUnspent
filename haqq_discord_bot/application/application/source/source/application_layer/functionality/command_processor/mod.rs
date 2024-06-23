pub mod run_bot;

use std::marker::PhantomData;

pub struct CommandProcessor<S> {
    _subject: PhantomData<S>,
}

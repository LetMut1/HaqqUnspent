pub mod create_fixtures;
pub mod run_server;

use std::marker::PhantomData;

pub struct CommandProcessor<S> {
    _subject: PhantomData<S>,
}

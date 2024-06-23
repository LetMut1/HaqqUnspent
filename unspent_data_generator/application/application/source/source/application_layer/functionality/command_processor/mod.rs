pub mod run_all_tasks;

use std::marker::PhantomData;

pub struct CommandProcessor<S> {
    _subject: PhantomData<S>,
}

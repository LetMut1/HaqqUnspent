pub mod tokio_blocking_task;
pub mod tokio_non_blocking_task;

use std::marker::PhantomData;

pub struct Spawner<S> {
    _subject: PhantomData<S>,
}

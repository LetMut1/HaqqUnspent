pub mod cron_job___task;

use std::marker::PhantomData;

pub struct Runner<S> {
    _subject: PhantomData<S>,
}

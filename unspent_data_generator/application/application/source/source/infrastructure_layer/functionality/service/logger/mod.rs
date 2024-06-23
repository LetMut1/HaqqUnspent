pub mod action_round___error_auditor;
pub mod action_round___invalid_argument_auditor;
pub mod action_round___response;
pub mod common;
pub mod cron_job___task;
pub mod error_auditor;

use std::marker::PhantomData;

pub struct Logger<S> {
    _subject: PhantomData<S>,
}

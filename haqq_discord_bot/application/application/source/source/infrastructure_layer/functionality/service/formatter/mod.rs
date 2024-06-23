pub mod action_round;
pub mod backtrace;
pub mod error_auditor;
pub mod invalid_argument_auditor;

use std::marker::PhantomData;

pub struct Formatter<S> {
    _subject: PhantomData<S>,
}

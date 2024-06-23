pub mod action_round___error_auditor;
pub mod action_round___invalid_argument;
pub mod action_round___response;

use std::marker::PhantomData;

pub struct Reactor<S> {
    _subject: PhantomData<S>,
}

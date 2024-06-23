pub mod cancel_raffle;
pub mod complete_raffle;
pub mod generalized_action;
pub mod run_bot;
pub mod run_server;
pub mod serve_raffle;
pub mod update_discord_roles;
pub mod verify_wallet;
pub mod update_raffle;

use std::marker::PhantomData;

pub struct Processor<S> {
    _subject: PhantomData<S>,
}

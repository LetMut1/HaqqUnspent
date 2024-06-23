pub mod discord_composite_custom_id;
pub mod environment_configuration;
pub mod qr_code;
pub mod serenity_http;
pub mod utc_date_time;

use std::marker::PhantomData;

pub struct Resolver<S> {
    _subject: PhantomData<S>,
}

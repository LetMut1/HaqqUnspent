pub mod request_parts;
pub mod server_access_token;

use std::marker::PhantomData;

pub struct Validator<S> {
    _subject: PhantomData<S>,
}

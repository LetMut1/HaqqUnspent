pub mod address_verification_data__expired_at;

use std::marker::PhantomData;

pub struct Creator<S> {
    _subject: PhantomData<S>,
}

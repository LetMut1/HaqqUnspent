use std::error::Error as StdError;

pub enum Error {
    Logic {
        message: &'static str,
    },
    Runtime {
        runtime: Runtime,
    },
}

impl Error {
    pub fn create_unreachable_state() -> Self {
        return Self::Logic {
            message: "Unreachable state.",
        };
    }

    pub fn create_incoming_invalid_state() -> Self {
        return Self::Logic {
            message: "The action processor Incoming in invalid state.",
        };
    }
}

pub enum Runtime {
    Other {
        other: Other,
    },
    Resource {
        resource: Resource,
    },
}

pub struct Other {
    error: Box<dyn StdError + Send + Sync + 'static>,
}

impl Other {
    pub fn new<E>(error: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        return Self {
            error: error.into(),
        };
    }

    pub fn new_(error: Box<dyn StdError + Send + Sync + 'static>) -> Self {
        return Self {
            error,
        };
    }

    pub fn get_error<'a>(&'a self) -> &'a (dyn StdError + 'static) {
        return self.error.as_ref();
    }
}

pub enum Resource {
    Clickhouse,
}

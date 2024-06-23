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

    pub fn create_value_does_not_exist() -> Self {
        return Self::Logic {
            message: "Value does not exist.",
        };
    }

    pub fn create_value_already_exist() -> Self {
        return Self::Logic {
            message: "Value already exist.",
        };
    }

    pub fn create_invalid_value() -> Self {
        return Self::Logic {
            message: "Invalid value.",
        };
    }

    pub fn create_overflow_occured() -> Self {
        return Self::Runtime {
            runtime: Runtime::Other {
                other: Other::new_("The overflow occured.".into()),
            },
        };
    }
}

pub enum Runtime {
    Other {
        other: Other,
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

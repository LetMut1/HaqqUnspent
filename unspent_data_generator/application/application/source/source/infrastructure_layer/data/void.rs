use serde::{
    Deserialize,
    Serialize,
};
use std::{
    error::Error,
    fmt::{
        Debug,
        Display,
        Error as FmtError,
        Formatter,
    },
};

#[derive(Serialize, Deserialize)]
pub enum Void {}

pub struct ErrorVoid(Void);

impl Debug for ErrorVoid {
    fn fmt<'a>(&'a self, _: &'a mut Formatter<'_>) -> Result<(), FmtError> {
        return Err(FmtError);
    }
}

impl Display for ErrorVoid {
    fn fmt<'a>(&'a self, _: &'a mut Formatter<'_>) -> Result<(), FmtError> {
        return Err(FmtError);
    }
}

impl Error for ErrorVoid {}

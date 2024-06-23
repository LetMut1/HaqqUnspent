use serde::Serialize;
use std::{
    error::Error,
    fmt::{
        Debug,
        Display,
        Error as FmtError,
        Formatter,
    },
};

#[derive(Serialize)]
pub enum Void {}

pub struct ErrorVoid(Void);

impl Error for ErrorVoid {}

impl Display for ErrorVoid {
    fn fmt<'a>(&'a self, _: &'a mut Formatter<'_>) -> Result<(), FmtError> {
        return Err(FmtError);
    }
}

impl Debug for ErrorVoid {
    fn fmt<'a>(&'a self, _: &'a mut Formatter<'_>) -> Result<(), FmtError> {
        return Err(FmtError);
    }
}

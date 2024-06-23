use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

#[derive(Serialize, Deserialize)]
pub enum Void {}

pub struct ErrorVoid(Void);

impl Debug for ErrorVoid {
    fn fmt<'a>(
        &'a self,
        _: &'a mut Formatter<'_>,
    ) -> Result<(), FmtError> {
        return Err(FmtError);
    }
}

impl Display for ErrorVoid {
    fn fmt<'a>(
        &'a self,
        _: &'a mut Formatter<'_>,
    ) -> Result<(), FmtError> {
        return Err(FmtError);
    }
}

impl Error for ErrorVoid {}

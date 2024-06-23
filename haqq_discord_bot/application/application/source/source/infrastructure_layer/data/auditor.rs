use super::backtrace::Backtrace;
use super::backtrace::BacktracePart;
use super::error::Error;
use std::error::Error as StdError;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

pub struct Auditor<T> {
    subject: T,
    backtrace: Backtrace,
}

impl<T> Auditor<T> {
    pub fn new(
        subject: T,
        backtrace_part: BacktracePart,
    ) -> Self {
        return Self {
            subject,
            backtrace: Backtrace::new(backtrace_part),
        };
    }

    pub fn add_backtrace_part<'a>(
        &'a mut self,
        backtrace_part: BacktracePart,
    ) -> () {
        self.backtrace.add(backtrace_part);

        return ();
    }

    pub fn get_subject<'a>(&'a self) -> &'a T {
        return &self.subject;
    }

    pub fn get_backtrace<'a>(&'a self) -> &'a Backtrace {
        return &self.backtrace;
    }
}

impl Debug for Auditor<Error> {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>,
    ) -> Result<(), FmtError> {
        return Ok(());
    }
}

impl Display for Auditor<Error> {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>,
    ) -> Result<(), FmtError> {
        return Ok(());
    }
}

impl StdError for Auditor<Error> {}

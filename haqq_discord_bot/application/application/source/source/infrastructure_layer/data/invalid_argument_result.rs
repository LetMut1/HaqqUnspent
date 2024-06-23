use super::auditor::Auditor;
use super::invalid_argument::InvalidArgument;

pub enum InvalidArgumentResult<T> {
    Ok {
        subject: T,
    },
    InvalidArgumentAuditor {
        invalid_argument_auditor: Auditor<InvalidArgument>,
    },
}

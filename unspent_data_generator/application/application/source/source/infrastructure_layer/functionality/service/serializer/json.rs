use super::{
    Serialize,
    Serializer,
};
pub use crate::infrastructure_layer::data::control_type::Json;
use crate::infrastructure_layer::data::{
    auditor::Auditor,
    backtrace::BacktracePart,
    error::{
        Error,
        Other,
        Runtime,
    },
};
use serde::{
    Deserialize,
    Serialize as SerdeSerialize,
};
use serde_json;

impl Serialize for Serializer<Json> {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, Auditor<Error>>
    where
        T: SerdeSerialize,
    {
        let data = match serde_json::to_vec(subject) {
            Ok(data_) => data_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        return Ok(data);
    }

    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, Auditor<Error>>
    where
        T: Deserialize<'a>,
    {
        let subject = match serde_json::from_slice::<'_, T>(data) {
            Ok(subject_) => subject_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        return Ok(subject);
    }
}

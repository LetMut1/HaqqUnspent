use super::Extractor;
pub use crate::infrastructure_layer::data::control_type::HttpBodyData;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        error::{
            Error,
            Other,
            Runtime,
        },
        invalid_argument_result::InvalidArgumentResult,
    },
    functionality::service::serializer::{
        Serialize,
        Serializer,
    },
};
use bytes::Buf;
use http::request::Parts;
use hyper::{
    body::to_bytes,
    Body,
};
use matchit::Params;
use serde::Deserialize;

impl Extractor<HttpBodyData> {
    pub async fn extract<'a, D, SF>(body: &'a mut Body, _parts: &'a Parts, _route_parameters: &'a Params<'_, '_>) -> Result<InvalidArgumentResult<Option<D>>, Auditor<Error>>
    where
        D: for<'de> Deserialize<'de>,
        Serializer<SF>: Serialize,
    {
        let bytes = match to_bytes(body).await {
            Ok(bytes_) => bytes_,
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

        let data = match Serializer::<SF>::deserialize::<'_, D>(bytes.chunk()) {
            Ok(data_) => data_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: Some(data),
            },
        );
    }
}

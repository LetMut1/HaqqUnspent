use super::Extractor;
pub use crate::infrastructure_layer::data::control_type::HttpBodyData;
use crate::infrastructure_layer::data::{
    auditor::Auditor,
    error::Error,
    invalid_argument_result::InvalidArgumentResult,
    void::Void,
};
use http::request::Parts;
use hyper::Body;
use matchit::Params;

impl Extractor<Void> {
    pub async fn extract<'a>(_body: &'a mut Body, _parts: &'a Parts, _route_parameters: &'a Params<'_, '_>) -> Result<InvalidArgumentResult<Option<Void>>, Auditor<Error>> {
        return Ok(
            InvalidArgumentResult::Ok {
                subject: None,
            },
        );
    }
}

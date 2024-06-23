use super::Validator;
use crate::infrastructure_layer::data::control_type::Response;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use http::header::CONTENT_LENGTH;
use http::header::CONTENT_TYPE;
pub use http::request::Parts;

pub use crate::infrastructure_layer::data::control_type::Request;

impl Validator<Parts> {
    pub fn is_valid<'a>(parts: &'a Parts) -> bool {
        let header_map = &parts.headers;

        let header_value_content_type = match header_map.get(CONTENT_TYPE) {
            Some(header_value_content_type_) => header_value_content_type_,
            None => {
                return false;
            }
        };

        if header_value_content_type.as_bytes() != Creator::<Response>::HEADER_VALUE_CONTENT_TYPE.as_bytes() {
            return false;
        }

        if let None = header_map.get(CONTENT_LENGTH) {
            return false;
        }

        return true;
    }
}

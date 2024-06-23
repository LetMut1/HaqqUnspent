use super::Creator;
use http::header;
use http::HeaderMap;
use http::HeaderValue;
use http::Version;
use hyper::Body;
use hyper::Response as HyperResponse;
use hyper::StatusCode;
use std::convert::From;

pub use crate::infrastructure_layer::data::control_type::Response;

impl Creator<Response> {
    pub const HEADER_VALUE_CONTENT_TYPE: &'static str = "application/octet-stream";

    fn create(
        status_code: StatusCode,
        data: Option<Vec<u8>>,
    ) -> Response {
        let mut header_map = HeaderMap::new();

        header_map.append(
            header::CONTENT_TYPE,
            HeaderValue::from_static(Self::HEADER_VALUE_CONTENT_TYPE),
        );

        let mut parts = HyperResponse::new(()).into_parts().0;

        parts.status = status_code;

        parts.version = Version::HTTP_2;

        parts.headers = header_map;

        let body = match data {
            Some(data_) => Body::from(data_),
            None => Body::empty(),
        };

        return Response::from_parts(
            parts, body,
        );
    }

    pub fn create_bad_request() -> Response {
        return Self::create(
            StatusCode::BAD_REQUEST,
            None,
        );
    }

    pub fn create_unauthorized() -> Response {
        return Self::create(
            StatusCode::UNAUTHORIZED,
            None,
        );
    }

    pub fn create_not_found() -> Response {
        return Self::create(
            StatusCode::NOT_FOUND,
            None,
        );
    }

    pub fn create_internal_server_error() -> Response {
        return Self::create(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
        );
    }

    pub fn create_not_extended() -> Response {
        return Self::create(
            StatusCode::NOT_EXTENDED,
            None,
        );
    }

    pub fn create_ok(data: Vec<u8>) -> Response {
        return Self::create(
            StatusCode::OK,
            Some(data),
        );
    }

    pub fn create_ok_() -> Response {
        return Self::create(
            StatusCode::OK,
            None,
        );
    }
}

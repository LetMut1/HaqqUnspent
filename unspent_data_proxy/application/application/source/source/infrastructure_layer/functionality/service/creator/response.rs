use super::Creator;
pub use crate::infrastructure_layer::data::control_type::Response;
use http::{
    header,
    HeaderMap,
    HeaderValue,
    Version,
};
use hyper::{
    Body,
    Response as HyperResponse,
    StatusCode,
};
use std::convert::From;

impl Creator<Response> {
    pub const HEADER_VALUE_CONTENT_TYPE: &'static str = "application/octet-stream";

    fn create(status_code: StatusCode, data: Option<Vec<u8>>) -> Response {
        let mut header_map = HeaderMap::new();

        header_map.append(
            header::CONTENT_TYPE,
            HeaderValue::from_static(Self::HEADER_VALUE_CONTENT_TYPE),
        );

        header_map.append(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        );

        header_map.append(
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            HeaderValue::from_static("false"),
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
            parts,
            body,
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

    pub fn create_ok(data: Option<Vec<u8>>) -> Response {
        return Self::create(
            StatusCode::OK,
            data,
        );
    }

    pub fn create_cors_ok() -> Response {
        let mut header_map = HeaderMap::new();

        header_map.append(
            header::CONTENT_TYPE,
            HeaderValue::from_static(Self::HEADER_VALUE_CONTENT_TYPE),
        );

        header_map.append(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        );

        header_map.append(
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            HeaderValue::from_static("false"),
        );

        header_map.append(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("POST"),
        );

        header_map.append(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("*"),
        );

        header_map.append(
            header::ACCESS_CONTROL_MAX_AGE,
            HeaderValue::from_static("600"),
        );

        let mut parts = HyperResponse::new(()).into_parts().0;

        parts.status = StatusCode::OK;

        parts.version = Version::HTTP_2;

        parts.headers = header_map;

        let body = Body::empty();

        return Response::from_parts(
            parts,
            body,
        );
    }
}

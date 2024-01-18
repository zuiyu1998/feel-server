use poem::{Body, Error, Response};
use poem_openapi::{
    error::ParseRequestPayloadError,
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse, Object,
};

const OK_CODE: i32 = 10000;
const INVALID_REQUEST_CODE: i32 = 400;
const UNKNOWN_CODE: i32 = 415;

use crate::ServerError;

use super::middleware::MiddlewareKind;

#[derive(ApiResponse)]
#[oai(bad_request_handler = "inline_bad_request_handler")]
pub enum GenericApiResponse<T: ParseFromJSON + ToJSON + Send + Sync> {
    #[oai(status = 200)]
    Ok(Json<ResponseObject<T>>),
}
#[derive(Debug, Object)]
pub struct EmptyRespone;

fn inline_bad_request_handler<T: ParseFromJSON + ToJSON + Send + Sync>(
    err: Error,
) -> GenericApiResponse<T> {
    GenericApiResponse::Ok(Json(bad_request_handler(err)))
}

#[derive(Object)]
pub struct ResponseObject<T: ParseFromJSON + ToJSON + Send + Sync> {
    code: i32,
    msg: String,
    data: Option<T>,
}

impl<T: ParseFromJSON + ToJSON + Send + Sync> ResponseObject<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: OK_CODE,
            msg: "ok".to_string(),
            data: Some(data),
        }
    }
}

pub fn bad_request_handler<T: ParseFromJSON + ToJSON + Send + Sync>(
    err: Error,
) -> ResponseObject<T> {
    if err.is::<ParseRequestPayloadError>() {
        ResponseObject {
            code: INVALID_REQUEST_CODE,
            msg: err.to_string(),
            data: None,
        }
    } else {
        ResponseObject {
            code: UNKNOWN_CODE,
            msg: err.to_string(),
            data: None,
        }
    }
}

pub fn bad_middleware_response(err: MiddlewareKind) -> Response {
    let object = bad_middleware_response_handler::<EmptyRespone>(err);
    let json = object.to_json().unwrap();

    let body = Body::from_json(json).unwrap();

    Response::from(body)
}

pub fn bad_middleware_response_handler<T: ParseFromJSON + ToJSON + Send + Sync>(
    err: MiddlewareKind,
) -> ResponseObject<T> {
    let code: i32;
    let msg: String;

    match err {
        _ => {
            code = OK_CODE;
            msg = err.to_string()
        }
    }

    ResponseObject {
        code,
        msg,
        data: None,
    }
}

pub fn bad_response_handler<T: ParseFromJSON + ToJSON + Send + Sync>(
    err: ServerError,
) -> ResponseObject<T> {
    let code: i32;
    let msg: String;

    match err {
        _ => {
            code = OK_CODE;
            msg = err.to_string()
        }
    }

    ResponseObject {
        code,
        msg,
        data: None,
    }
}

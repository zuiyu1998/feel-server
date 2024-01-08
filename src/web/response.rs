use poem::Error;
use poem_openapi::{
    error::ParseRequestPayloadError,
    types::{ParseFromJSON, ToJSON},
    Object,
};
const OK_CODE: i32 = 10000;
const INVALID_REQUEST_CODE: i32 = 400;
const UNKNOWN_CODE: i32 = 415;

use crate::ServerError;

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

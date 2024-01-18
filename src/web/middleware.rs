use poem::{
    async_trait,
    error::ResponseError,
    http::{header, HeaderName},
    Endpoint, IntoResponse, Middleware, Request, Response, Result,
};
use std::result::Result as StdResult;
use thiserror::Error;

use crate::helper::JwtHelper;

#[derive(Debug, Error)]
pub enum MiddlewareKind {
    #[error("HeaderValueNotFound")]
    HeaderValueNotFound,
    #[error("HeaderValueInvaild")]
    HeaderValueInvaild,
}

impl ResponseError for MiddlewareKind {
    fn status(&self) -> poem::http::StatusCode {
        poem::http::StatusCode::OK
    }
}
#[derive(Clone)]
pub struct PermissionMiddleware {
    pub header_name: HeaderName,
}

impl Default for PermissionMiddleware {
    fn default() -> Self {
        PermissionMiddleware {
            header_name: header::AUTHORIZATION,
        }
    }
}

impl<E: Endpoint> Middleware<E> for PermissionMiddleware {
    type Output = PermissionMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        PermissionMiddlewareImpl {
            ep,
            middleware: self.clone(),
        }
    }
}

impl PermissionMiddleware {
    pub fn get_user_id(&self, req: &Request) -> StdResult<i32, MiddlewareKind> {
        let header_value = req
            .header(&self.header_name)
            .ok_or(MiddlewareKind::HeaderValueNotFound)?;

        let mut split_n = header_value.splitn(2, " ");
        split_n.next();

        if let Some(token) = split_n.next() {
            let helper = req.extensions().get::<JwtHelper>();

            if let Some(helper) = helper {
                let user_id = helper
                    .decode(token)
                    .map_err(|_| MiddlewareKind::HeaderValueInvaild)?
                    .parse::<i32>()
                    .map_err(|_| MiddlewareKind::HeaderValueInvaild)?;

                return Ok(user_id);
            } else {
                return Err(MiddlewareKind::HeaderValueInvaild);
            }
        } else {
            return Err(MiddlewareKind::HeaderValueInvaild);
        }
    }
}

pub struct PermissionMiddlewareImpl<E> {
    ep: E,
    middleware: PermissionMiddleware,
}

impl<E> PermissionMiddlewareImpl<E> {}

#[async_trait]
impl<E: Endpoint> Endpoint for PermissionMiddlewareImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        self.middleware.get_user_id(&req)?;

        let res = self.ep.call(req).await?;

        Ok(res.into_response())
    }
}

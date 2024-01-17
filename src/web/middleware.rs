use poem::{
    async_trait,
    error::ResponseError,
    http::{header, HeaderName},
    Endpoint, IntoResponse, Middleware, Request, Response, Result,
};
use std::result::Result as StdResult;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MiddlewareKind {}

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
    pub fn get_user_id(&self, _req: &Request) -> StdResult<i32, MiddlewareKind> {
        return Ok(1);
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

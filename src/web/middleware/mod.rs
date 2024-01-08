use thiserror::Error;

mod auth;

#[derive(Debug, Error)]
pub enum MiddlewareKind {
    #[error("HeaderValueNotFound")]
    HeaderValueNotFound,
    #[error("HeaderValueScheme")]
    HeaderValueScheme,
}

use rc_entity::sea_orm::DbErr;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    DbErr(#[from] DbErr),
    #[error(transparent)]
    IoError(#[from] IoError),
}

pub type ServerResult<T, E = ServerError> = std::result::Result<T, E>;

use figment::Error as FigmentError;
use rc_entity::sea_orm::DbErr;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    DbErr(#[from] DbErr),
    #[error(transparent)]
    IoError(#[from] IoError),
    #[error(transparent)]
    FigmentError(#[from] FigmentError),
}

pub type ServerResult<T, E = ServerError> = std::result::Result<T, E>;

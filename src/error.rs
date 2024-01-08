use figment::Error as FigmentError;
use rc_entity::sea_orm::DbErr;
use rc_storage::StorageError;
use std::io::Error as IoError;
use thiserror::Error;

use crate::jwt_helper::JwtKind;

#[derive(Debug, Error)]

pub enum ServerKind {
    #[error("SettingValueNotFound")]
    SettingValueNotFound,
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    DbErr(#[from] DbErr),
    #[error(transparent)]
    IoError(#[from] IoError),
    #[error(transparent)]
    FigmentError(#[from] FigmentError),
    #[error(transparent)]
    StorageError(#[from] StorageError),
    #[error(transparent)]
    Kind(#[from] ServerKind),
    #[error(transparent)]
    JwtKind(#[from] JwtKind),
}

pub type ServerResult<T, E = ServerError> = std::result::Result<T, E>;

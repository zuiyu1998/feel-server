use rc_entity::sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageKind {
    #[error("AuthNotFound")]
    AuthNotFound,
    #[error("FollowDetailNotFound")]
    FollowDetailNotFound,
    #[error("AuthExist")]
    AuthExist,
    #[error("AuthExist")]
    UserExist,
    #[error("UserNotFound")]
    UserNotFound,
    #[error("PasswordError")]
    PasswordError,
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error(transparent)]
    Kind(#[from] StorageKind),
    #[error(transparent)]
    DbErr(#[from] DbErr),
}

pub type StorageResult<T, E = StorageError> = std::result::Result<T, E>;

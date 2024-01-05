use rc_entity::sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error(transparent)]
    DbErr(#[from] DbErr),
}

pub type StorageResult<T, E = StorageError> = std::result::Result<T, E>;

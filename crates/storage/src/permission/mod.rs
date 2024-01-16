mod dto;

pub use dto::*;
use rc_entity::sea_orm::ConnectionTrait;

use crate::StorageResult;

pub struct PermissionStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> PermissionStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        Self { conn }
    }

    pub fn get_user_permissions(&self, user_id: i32) -> StorageResult<UserPermissions> {
        todo!()
    }
}

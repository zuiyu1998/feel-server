mod dto;

pub use dto::*;

use rc_entity::prelude::{UrlPermissionColumn, UrlPermissionEntity};
use rc_entity::sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
};

use crate::StorageResult;

pub struct PermissionStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> PermissionStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        Self { conn }
    }

    pub fn get_user_permissions(&self, _user_id: i32) -> StorageResult<UserPermissions> {
        todo!()
    }

    pub async fn get_all_url_permission(&self) -> StorageResult<Vec<UrlPermission>> {
        let sql = UrlPermissionEntity::find()
            .filter(UrlPermissionColumn::IsDelete.eq(false))
            .filter(UrlPermissionColumn::IsEnable.eq(true));

        let models = sql.all(self.conn).await?;

        let result = models
            .into_iter()
            .map(|model| UrlPermission::from(model))
            .collect::<Vec<UrlPermission>>();

        Ok(result)
    }

    pub async fn create_url_permission(
        &self,
        form: UrlPermissionForm,
    ) -> StorageResult<UrlPermission> {
        let active = form.get_active_model();

        let model = active.insert(self.conn).await?;

        Ok(UrlPermission::from(model))
    }
}

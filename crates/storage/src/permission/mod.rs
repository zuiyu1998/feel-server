mod dto;

pub use dto::*;

use rc_entity::sea_orm::{ActiveModelTrait, ConnectionTrait, FromQueryResult, Statement};

use rc_entity::prelude::UrlPermissionModel;

use crate::{StorageResult, DATABASEBACKEND};

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
        let stmt = Statement::from_sql_and_values(
            DATABASEBACKEND,
            r#"
            select
            pup.id as id,
            pup.permission_id = permission_id,
            pup.url = url,
            pup.create_at as create_at,
            pup.update_at as update_at,
            pp."name" as "name"
            from pb_url_permission pup 
            left join pb_permission pp  on pp.id = pup.permission_id  
            where pup.is_delete = false and pup.is_enable = true
        "#,
            vec![],
        );

        let data = UrlPermission::find_by_statement(stmt)
            .all(self.conn)
            .await?;

        Ok(data)
    }

    pub async fn create_url_permission(
        &self,
        form: UrlPermissionForm,
    ) -> StorageResult<UrlPermissionModel> {
        let active = form.get_active_model();

        let model = active.insert(self.conn).await?;

        Ok(model)
    }
}

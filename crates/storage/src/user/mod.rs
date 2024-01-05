use rc_entity::sea_orm::{ActiveModelTrait, ConnectionTrait, IntoActiveModel, Set};

mod dto;

pub use dto::*;

use crate::StorageResult;

pub struct UserStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> UserStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        UserStorage { conn }
    }

    pub async fn create_user(&self, form: UserFormEncrypt) -> StorageResult<User> {
        let auth = form.get_user_auth_active_model();

        let auth = auth.insert(self.conn).await?;

        let user = form.get_user_base_active_model();

        let user = user.insert(self.conn).await?;

        let mut auth_active = auth.into_active_model();

        auth_active.user_id = Set(user.id);

        auth_active.update(self.conn).await?;

        Ok(User::from(user))
    }
}

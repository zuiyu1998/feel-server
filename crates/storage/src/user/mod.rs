use rc_entity::sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set,
};

use rc_entity::prelude::{
    UserAuthClass, UserAuthColumn, UserAuthEntity, UserAuthModel, UserBaseColumn, UserBaseEntity,
    UserBaseModel,
};

mod dto;

pub use dto::*;

use crate::{StorageKind, StorageResult};

pub struct UserStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> UserStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        UserStorage { conn }
    }

    pub async fn find_user_base(
        &self,
        option: UserBaseOption,
    ) -> StorageResult<Option<UserBaseModel>> {
        let mut sql = UserBaseEntity::find();

        if let Some(nikename) = option.nikename {
            sql = sql.filter(UserBaseColumn::Nikename.eq(nikename));
        }

        if let Some(uid) = option.uid {
            sql = sql.filter(UserBaseColumn::Uid.eq(uid));
        }

        let user = sql.one(self.conn).await?;

        Ok(user)
    }

    pub async fn find_user_auth(
        &self,
        option: UserAuthOption,
    ) -> StorageResult<Option<UserAuthModel>> {
        let mut sql = UserAuthEntity::find();

        if let Some(user_id) = option.user_id {
            sql = sql.filter(UserAuthColumn::UserId.eq(user_id));
        }

        if let Some(auth_class) = option.auth_class {
            sql = sql.filter(UserAuthColumn::AuthClass.eq(UserAuthClass::from(auth_class)));
        }

        if let Some(unique_name) = option.unique_name {
            sql = sql.filter(UserAuthColumn::UniqueName.eq(unique_name));
        }

        let auth = sql.one(self.conn).await?;

        Ok(auth)
    }

    pub async fn create_user(&self, form: UserFormEncrypt) -> StorageResult<User> {
        let mut option = UserAuthOption::default();
        option.auth_class = Some(form.auth_class.clone());
        option.unique_name = Some(form.auth_name.clone());

        if let Some(_) = self.find_user_auth(option).await? {
            return Err(StorageKind::AuthExist.into());
        }

        let mut option = UserBaseOption::default();
        option.uid = Some(form.uid.clone());
        option.nikename = Some(form.nikename.clone());

        if let Some(_) = self.find_user_base(option).await? {
            return Err(StorageKind::UserExist.into());
        }

        let user = form.get_user_base_active_model();

        let user = user.insert(self.conn).await?;

        let mut auth = form.get_user_auth_active_model();

        auth.user_id = Set(user.id);

        auth.insert(self.conn).await?;

        Ok(User::from(user))
    }
}

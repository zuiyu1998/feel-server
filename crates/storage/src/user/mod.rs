use rc_entity::sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, QueryFilter, Set,
    Statement,
};

use rc_entity::prelude::{
    UserAuthClass, UserAuthColumn, UserAuthEntity, UserAuthModel, UserBaseColumn, UserBaseEntity,
    UserBaseModel,
};

mod dto;

pub use dto::*;

use crate::{StorageKind, StorageResult, DATABASEBACKEND};

pub struct UserStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> UserStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        UserStorage { conn }
    }

    pub async fn find(&self, user_id: i32) -> StorageResult<UserDetail> {
        let stmt = Statement::from_sql_and_values(
            DATABASEBACKEND,
            r#"
            select
            pub.id as id,
            pub.nikename as nikename,
            pub.uid as uid,
            pub.avatar as avatar,
            pub.create_at as create_at,
            pub.update_at as update_at,
            pufd.like_count as like_count,
            pufd.follow_count as follow_count
            from pb_user_base pub 
            join pb_user_follow_detail pufd on pufd.user_id = pub.id
            where user_id = $1
        "#,
            vec![user_id.into()],
        );

        let user = UserDetail::find_by_statement(stmt).one(self.conn).await?;

        if user.is_none() {
            return Err(StorageKind::UserNotFound.into());
        }

        let user = user.unwrap();

        Ok(user)
    }

    pub async fn login(&self, form: UserLoginFormEncrypt) -> StorageResult<User> {
        let auth_option = form.get_user_auth_option();

        let auth = self.find_user_auth(auth_option).await?;

        if auth.is_none() {
            return Err(StorageKind::AuthNotFound.into());
        }

        let auth = auth.unwrap();

        if auth.auth_data != form.auth_data {
            return Err(StorageKind::PasswordError.into());
        }

        let user = UserBaseEntity::find_by_id(auth.user_id)
            .one(self.conn)
            .await?;

        if user.is_none() {
            return Err(StorageKind::UserNotFound.into());
        }

        Ok(User::from(user.unwrap()))
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

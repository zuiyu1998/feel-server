use chrono::NaiveDateTime;
use rc_entity::{
    prelude::{UserAuthActiveModel, UserAuthClass, UserBaseActiveModel, UserBaseModel},
    sea_orm::Set,
};

use crate::utils::get_now;

pub struct User {
    pub id: i32,
    pub nikename: String,
    pub uid: String,
    pub avatar: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

impl From<UserBaseModel> for User {
    fn from(value: UserBaseModel) -> Self {
        let UserBaseModel {
            id,
            nikename,
            uid,
            avatar,
            create_at,
            update_at,
            ..
        } = value;

        User {
            id,
            nikename,
            uid,
            avatar,
            create_at,
            update_at,
        }
    }
}

pub struct UserForm {
    pub nikename: String,
    pub uid: String,
    pub avatar: String,
    pub auth_class: AuthClass,
    pub auth_name: String,
    pub auth_data: String,
}

pub struct UserFormEncrypt {
    pub nikename: String,
    pub uid: String,
    pub avatar: String,
    pub auth_class: AuthClass,
    pub auth_name: String,
    pub auth_data: Vec<u8>,
}

impl UserFormEncrypt {
    pub fn from_form(form: UserForm, encrypt_data: Vec<u8>) -> UserFormEncrypt {
        let UserForm {
            nikename,
            uid,
            avatar,
            auth_class,
            auth_name,
            ..
        } = form;

        UserFormEncrypt {
            nikename,
            uid,
            avatar,
            auth_class,
            auth_name,
            auth_data: encrypt_data,
        }
    }

    pub fn get_user_auth_active_model(&self) -> UserAuthActiveModel {
        let mut active: UserAuthActiveModel = Default::default();

        let now = get_now();

        active.unique_name = Set(self.auth_name.clone());
        active.auth_data = Set(self.auth_data.clone());
        active.auth_class = Set(UserAuthClass::from(self.auth_class.clone()));
        active.create_at = Set(now.clone());
        active.update_at = Set(now);

        active
    }

    pub fn get_user_base_active_model(&self) -> UserBaseActiveModel {
        let mut active: UserBaseActiveModel = Default::default();

        let now = get_now();

        active.nikename = Set(self.nikename.clone());
        active.uid = Set(self.uid.clone());
        active.avatar = Set(self.avatar.clone());
        active.create_at = Set(now.clone());
        active.update_at = Set(now);

        active
    }
}

#[derive(Debug, Clone)]
pub enum AuthClass {
    Email,
}

impl From<UserAuthClass> for AuthClass {
    fn from(value: UserAuthClass) -> Self {
        match value {
            UserAuthClass::Email => AuthClass::Email,
        }
    }
}

impl From<AuthClass> for UserAuthClass {
    fn from(value: AuthClass) -> Self {
        match value {
            AuthClass::Email => UserAuthClass::Email,
        }
    }
}

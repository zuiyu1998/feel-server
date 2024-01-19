use rc_entity::chrono::NaiveDateTime;
use rc_entity::prelude::{get_now, UrlPermissionActiveModel, UrlPermissionModel};
use rc_entity::sea_orm::{self, FromQueryResult, Set};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

pub struct UrlPermissionForm {
    pub url: String,
    pub permisssion_id: i32,
}

impl UrlPermissionForm {
    pub fn get_active_model(&self) -> UrlPermissionActiveModel {
        let mut active: UrlPermissionActiveModel = Default::default();

        let now = get_now();

        active.create_at = Set(now.clone());
        active.update_at = Set(now.clone());
        active.permission_id = Set(self.permisssion_id);
        active.url = Set(self.url.clone());

        active
    }
}

#[derive(Debug, FromQueryResult)]
pub struct UrlPermission {
    pub id: i32,
    pub permission_id: i32,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub url: String,
    pub name: String,
}

impl PartialEq<&str> for Permission {
    fn eq(&self, other: &&str) -> bool {
        if self.name == **other {
            return true;
        } else {
            return false;
        }
    }
}

pub struct UserPermissions(HashSet<String>);

impl UserPermissions {
    pub fn has_permission(&self, permission: &str) -> bool {
        self.0.contains(permission)
    }
}

pub struct Role {
    pub id: i32,
    pub name: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

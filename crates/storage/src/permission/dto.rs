use rc_entity::chrono::NaiveDateTime;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

pub struct UrlPermission {
    pub id: i32,
    pub name: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub url: String,
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

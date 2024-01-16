use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Permission {
    name: String,
}

pub struct UserPermissions(HashSet<Permission>);

impl UserPermissions {
    pub fn has_permission(&self, permission: &str) -> bool {
        let permission = Permission {
            name: permission.to_string(),
        };

        self.0.contains(&permission)
    }
}

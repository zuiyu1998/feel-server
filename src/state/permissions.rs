use crate::ServerResult;
use radix_tree::{Node, Radix};
use rc_entity::sea_orm::DatabaseConnection;
use rc_storage::prelude::PermissionStorage;

pub struct PermissionsManager(Node<char, String>);

impl PermissionsManager {
    pub async fn from_connection(conn: &DatabaseConnection) -> ServerResult<Self> {
        let mut node = Node::new("", None);

        let storage = PermissionStorage::new(conn);
        let permissions = storage.get_all_url_permission().await?;

        for permission in permissions.into_iter() {
            node.insert(permission.url, permission.name);
        }

        Ok(PermissionsManager(node))
    }

    pub fn get_url_permissions(&self, url: &str) -> Option<&str> {
        self.0
            .find(url.to_string())
            .and_then(|node| node.data.as_deref())
    }
}

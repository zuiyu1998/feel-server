use crate::ServerResult;
use radix_tree::{Node, Radix};
use rc_entity::sea_orm::DatabaseConnection;

pub struct PermissionsManager(Node<char, String>);

impl PermissionsManager {
    pub async fn from_connection(_conn: &DatabaseConnection) -> ServerResult<Self> {
        let mut node = Node::new("", None);

        Ok(PermissionsManager(node))
    }

    pub fn get_url_permissions(&self, url: &str) -> Option<&str> {
        self.0
            .find(url.to_string())
            .and_then(|node| node.data.as_deref())
    }
}

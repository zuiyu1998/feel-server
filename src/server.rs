use std::sync::Arc;

use crate::{Config, ServerResult};
use rc_entity::sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
pub struct Server {
    conn: DatabaseConnection,
    config: Arc<Config>,
}

impl Server {
    pub async fn from_config(config: &Arc<Config>) -> ServerResult<Self> {
        let conn = Database::connect(&config.server.database_url).await?;

        Ok(Server {
            conn,
            config: config.clone(),
        })
    }
}

async fn start_server(config: &Arc<Config>) -> ServerResult<Server> {
    Server::from_config(&config).await
}

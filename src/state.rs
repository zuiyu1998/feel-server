use std::sync::Arc;

use crate::{Config, ServerResult};
use rc_entity::sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
pub struct State {
    conn: DatabaseConnection,
    config: Arc<Config>,
}

impl State {
    pub async fn from_config(config: &Arc<Config>) -> ServerResult<Self> {
        let conn = Database::connect(&config.server.database_url).await?;

        Ok(State {
            conn,
            config: config.clone(),
        })
    }
}

pub async fn initialize(config: &Arc<Config>) -> ServerResult<State> {
    State::from_config(&config).await
}

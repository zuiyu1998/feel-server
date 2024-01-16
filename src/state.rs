use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    services::{PermissionsManager, ProjectSetting},
    Config, ServerResult,
};
use migration::{Migrator, MigratorTrait};
use rc_entity::sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
pub struct State {
    pub conn: DatabaseConnection,
    pub config: Arc<Config>,
    pub setting: Arc<RwLock<ProjectSetting>>,
    pub permissions_manager: Arc<RwLock<PermissionsManager>>,
}

impl State {
    pub async fn from_config(config: &Arc<Config>) -> ServerResult<Self> {
        let conn = Database::connect(&config.server.database_url).await?;

        Migrator::up(&conn, None).await?;

        let setting_raw = ProjectSetting::from_connection(&conn).await?;
        let permissions_raw = PermissionsManager::from_connection(&conn).await?;

        Ok(State {
            conn,
            config: config.clone(),
            setting: Arc::new(RwLock::new(setting_raw)),
            permissions_manager: Arc::new(RwLock::new(permissions_raw)),
        })
    }
}

pub async fn initialize(config: &Arc<Config>) -> ServerResult<State> {
    State::from_config(&config).await
}

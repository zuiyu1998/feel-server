use rc_entity::sea_orm::DatabaseConnection;
use rc_storage::prelude::{SettingStorage, SettingValue};
use std::collections::HashMap;

use crate::{state::State, ServerResult};

pub struct ProjectSetting(pub HashMap<String, SettingValue>);

impl ProjectSetting {
    pub async fn from_connection(conn: &DatabaseConnection) -> ServerResult<Self> {
        let mut map = HashMap::default();

        let storage = SettingStorage::new(conn);
        for value in storage.all().await?.into_iter() {
            map.insert(value.key.to_string(), value);
        }

        Ok(ProjectSetting(map))
    }
}

pub struct SettingService<'a> {
    state: &'a State,
}

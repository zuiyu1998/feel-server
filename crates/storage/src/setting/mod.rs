mod dto;

pub use dto::*;
use rc_entity::sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

use rc_entity::prelude::{SettingColumn, SettingEntity};

use crate::StorageResult;

pub struct SettingStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> SettingStorage<'a, C> {
    pub async fn all(&self) -> StorageResult<Vec<SettingValue>> {
        let sql = SettingEntity::find()
            .filter(SettingColumn::IsEnable.eq(true))
            .filter(SettingColumn::IsDelete.eq(false));

        let settings = sql
            .all(self.conn)
            .await?
            .into_iter()
            .map(|item| SettingValue::from(item))
            .collect::<Vec<SettingValue>>();

        Ok(settings)
    }
}

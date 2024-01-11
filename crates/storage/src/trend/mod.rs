use rc_entity::sea_orm::{ActiveModelTrait, ConnectionTrait};

mod dto;

pub use dto::*;

use crate::StorageResult;

pub struct TrendStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> TrendStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        TrendStorage { conn }
    }

    pub async fn create_trend(&self, form: TrendForm) -> StorageResult<Trend> {
        let active = form.get_trend_active_model();

        let model = active.insert(self.conn).await?;

        Ok(Trend::from(model))
    }
}

use rc_entity::prelude::{TrendColumn, TrendEntity};
use rc_entity::sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter,
};

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

    pub async fn get_list(&self, params: TrendParams) -> StorageResult<Vec<Trend>> {
        let mut sql = TrendEntity::find();

        if let Some(user_id) = params.user_id {
            sql = sql.filter(TrendColumn::UserId.eq(user_id));
        }

        let paginate = sql.paginate(self.conn, params.page_size);

        let trends = paginate
            .fetch_page(params.page)
            .await?
            .into_iter()
            .map(|item| Trend::from(item))
            .collect::<Vec<Trend>>();

        Ok(trends)
    }

    pub async fn create_trend(&self, form: TrendForm) -> StorageResult<Trend> {
        let active = form.get_trend_active_model();

        let model = active.insert(self.conn).await?;

        Ok(Trend::from(model))
    }
}

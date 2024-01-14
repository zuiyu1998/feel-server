use rc_entity::prelude::{TrendColumn, TrendEntity, TrendModel};
use rc_entity::sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, PaginatorTrait,
    QueryFilter, Statement,
};

mod dto;

pub use dto::*;

use crate::utils::MetaHelper;
use crate::{StorageResult, DATABASEBACKEND};

pub struct TrendStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> TrendStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        TrendStorage { conn }
    }

    pub async fn trend_update(&self, user_id: i32) -> StorageResult<()> {
        let form = TrendUpdateForm { user_id };
        let active = form.get_trend_update_active_model();

        active.update(self.conn).await?;

        Ok(())
    }

    pub async fn create_trend_update(&self, user_id: i32) -> StorageResult<()> {
        let form = TrendUpdateForm { user_id };

        let active = form.get_trend_update_active_model();

        active.insert(self.conn).await?;

        Ok(())
    }

    pub async fn get_follow_list(&self, params: TrendParams) -> StorageResult<Vec<TrendDetail>> {
        let user_id = params.user_id.clone();

        let stmt = Statement::from_sql_and_values(
            DATABASEBACKEND,
            r#"
            select
            pt.id as id,
            pt.user_id as user_id,
            pt."content" as "content",
            pt.meta_source as meta_source,
            pt.meta_soure_id as meta_soure_id,
            pt.create_at as create_at,
            pt.update_at as update_at,
            pt.like_count as like_count,
            pt.unlike_count as unlike_count,
            pt.is_delete as is_delete,
            pt.is_enable as is_enable
            from pb_user_follow puf
            left join pb_user_trend_update putu on putu.user_id = puf.owner_user_id 
            inner join pb_trend pt on pt.user_id  = puf.follow_user_id 
            where puf.owner_user_id  = $1
        "#,
            vec![user_id.unwrap().into()],
        );

        let sql = TrendModel::find_by_statement(stmt);

        let paginate = sql.paginate(self.conn, params.page_size);

        let trends = paginate
            .fetch_page(params.page)
            .await?
            .into_iter()
            .map(|item| Trend::from(item))
            .collect::<Vec<Trend>>();

        tracing::error!("len:{}", trends.len());

        let trend_details = MetaHelper::update_trends(self.conn, trends).await?;

        Ok(trend_details)
    }

    pub async fn get_list(&self, params: TrendParams) -> StorageResult<Vec<TrendDetail>> {
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

        let trend_details = MetaHelper::update_trends(self.conn, trends).await?;

        Ok(trend_details)
    }

    pub async fn create_trend(&self, form: TrendForm) -> StorageResult<Trend> {
        let active = form.get_trend_active_model();

        let model = active.insert(self.conn).await?;

        self.trend_update(model.user_id).await?;

        Ok(Trend::from(model))
    }
}

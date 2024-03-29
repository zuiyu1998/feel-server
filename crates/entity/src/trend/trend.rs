use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "pb_trend")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub create_at: ChronoDateTime,
    pub update_at: ChronoDateTime,
    pub is_delete: bool,
    pub is_enable: bool,
    pub like_count: i32,
    pub unlike_count: i32,
    pub meta_source: Option<TrendMetaSource>,
    pub meta_soure_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum TrendMetaSource {
    #[sea_orm(string_value = "Article")]
    Article,
}

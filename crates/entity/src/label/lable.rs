use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "pb_label")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub description: String,
    pub name: String,
    pub effect: i64,
    pub create_at: ChronoDateTime,
    pub update_at: ChronoDateTime,
    pub is_delete: bool,
    pub is_enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
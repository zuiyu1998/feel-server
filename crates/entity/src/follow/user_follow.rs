use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "pb_user_follow")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub owner_user_id: i32,
    pub follow_user_id: i32,
    pub create_at: ChronoDateTime,
    pub is_delete: bool,
    pub is_enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

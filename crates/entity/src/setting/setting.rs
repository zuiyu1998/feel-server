use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "pb_setting")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub key: String,
    pub raw_data: String,
    pub setting_data_type: SettingDataType,
    pub create_at: ChronoDateTime,
    pub update_at: ChronoDateTime,
    pub is_delete: bool,
    pub is_enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum SettingDataType {
    #[sea_orm(string_value = "Array")]
    Array,
}

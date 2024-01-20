use rc_entity::prelude::{get_now, ArticleActiveModel, ArticleModel};
use rc_entity::{chrono::NaiveDateTime, sea_orm::Set};

#[derive(Clone)]
pub struct Club {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub cover: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

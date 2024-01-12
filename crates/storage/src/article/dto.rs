use rc_entity::chrono::NaiveDateTime;

use crate::{
    prelude::{TrendForm, TrendMeta, TrendMetaSource},
    utils::RelatedThrend,
};

pub struct Article {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub background: String,
    pub content: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub like_count: i32,
    pub unlike_count: i32,
}

impl RelatedThrend for Article {
    fn get_thrend_form(&self) -> TrendForm {
        TrendForm {
            user_id: self.user_id,
            content: "发布了一篇文章".to_owned(),
            meta: Some(TrendMeta {
                source: TrendMetaSource::Article,
                source_id: self.id,
            }),
        }
    }
}

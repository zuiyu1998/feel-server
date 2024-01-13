use rc_entity::prelude::{get_now, ArticleActiveModel, ArticleModel};
use rc_entity::{chrono::NaiveDateTime, sea_orm::Set};

use crate::{
    prelude::TrendForm,
    utils::{MetaHelper, RelatedThrend},
};

pub struct ArticleForm {
    pub user_id: i32,
    pub title: String,
    pub cover: String,
    pub content: String,
}

impl ArticleForm {
    pub fn get_article_active_model(&self) -> ArticleActiveModel {
        let mut active = ArticleActiveModel::default();

        let now = get_now();
        active.user_id = Set(self.user_id);
        active.title = Set(self.title.clone());
        active.cover = Set(self.cover.clone());
        active.content = Set(self.content.clone());
        active.create_at = Set(now.clone());
        active.update_at = Set(now.clone());
        active.like_count = Set(0);
        active.unlike_count = Set(0);

        active
    }
}

#[derive(Clone)]
pub struct Article {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub cover: String,
    pub content: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub like_count: i32,
    pub unlike_count: i32,
}

impl From<ArticleModel> for Article {
    fn from(value: ArticleModel) -> Self {
        let ArticleModel {
            id,
            user_id,
            content,
            title,
            cover,
            create_at,
            update_at,
            like_count,
            unlike_count,
            ..
        } = value;

        Article {
            id,
            user_id,
            title,
            cover,
            content,
            create_at,
            update_at,
            like_count,
            unlike_count,
        }
    }
}

impl RelatedThrend for Article {
    fn get_thrend_form(&self) -> TrendForm {
        TrendForm {
            user_id: self.user_id,
            content: "发布了一篇文章".to_owned(),
            meta: Some(MetaHelper::get_article_trend_meta(self.id)),
        }
    }
}

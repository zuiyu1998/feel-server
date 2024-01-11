use rc_entity::chrono::NaiveDateTime;
use rc_entity::prelude::{get_now, TrendActiveModel, TrendModel};
use rc_entity::sea_orm::Set;

use crate::commit::CommitMeta;

pub struct TrendForm {
    pub user_id: i32,
    pub content: String,
    pub meta: Option<TrendMeta>,
}

impl TrendForm {
    pub fn get_trend_active_model(&self) -> TrendActiveModel {
        let mut active: TrendActiveModel = Default::default();

        let now = get_now();

        if let Some(meta) = self.meta.as_ref() {
            active.meta_soure_id = Set(Some(meta.source_id));
            active.meta_source = Set(Some(meta.source.clone()));
        }

        active.user_id = Set(self.user_id);
        active.content = Set(self.content.clone());
        active.update_at = Set(now.clone());
        active.create_at = Set(now.clone());
        active.like_count = Set(0);
        active.unlike_count = Set(0);

        active
    }
}

pub struct TrendMeta {
    pub source: String,
    pub source_id: i32,
}

pub struct Trend {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub meta: Option<TrendMeta>,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub like_count: i32,
    pub unlike_count: i32,
}

impl Trend {
    pub fn get_commit_meta(user_id: i32, source_id: i32) -> CommitMeta {
        CommitMeta {
            user_id,
            source: "trend".to_owned(),
            source_id,
        }
    }
}

impl From<TrendModel> for Trend {
    fn from(value: TrendModel) -> Self {
        let TrendModel {
            id,
            user_id,
            content,
            create_at,
            update_at,
            like_count,
            unlike_count,
            meta_source,
            meta_soure_id,
            ..
        } = value;

        let mut meta = None;

        if meta_source.is_some() && meta_soure_id.is_some() {
            meta = Some(TrendMeta {
                source: meta_source.unwrap(),
                source_id: meta_soure_id.unwrap(),
            });
        }

        Trend {
            id,
            user_id,
            content,
            meta,
            create_at,
            update_at,
            like_count,
            unlike_count,
        }
    }
}

use rc_entity::chrono::NaiveDateTime;
use rc_entity::prelude::TrendModel;

use crate::commit::CommitMeta;

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
    pub fn get_commit_meta(&self) -> CommitMeta {
        CommitMeta {
            user_id: self.user_id,
            source: "trend".to_owned(),
            source_id: self.id,
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

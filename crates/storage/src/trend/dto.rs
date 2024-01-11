use rc_entity::chrono::NaiveDateTime;
use rc_entity::prelude::{get_now, TrendActiveModel, TrendEntityMetaSource, TrendModel};
use rc_entity::sea_orm::Set;

use crate::commit::CommitMeta;
use crate::prelude::CommitMetaSource;
use crate::utils::MetaHelper;

pub struct TrendParams {
    pub page: u64,
    pub page_size: u64,
    pub user_id: Option<i32>,
}

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
            active.meta_source = Set(Some(TrendEntityMetaSource::from(meta.source.clone())));
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

#[derive(Debug, Clone)]
pub enum TrendMetaSource {
    Article,
}

impl From<TrendEntityMetaSource> for TrendMetaSource {
    fn from(value: TrendEntityMetaSource) -> Self {
        match value {
            TrendEntityMetaSource::Article => TrendMetaSource::Article,
        }
    }
}

impl From<TrendMetaSource> for TrendEntityMetaSource {
    fn from(value: TrendMetaSource) -> Self {
        match value {
            TrendMetaSource::Article => TrendEntityMetaSource::Article,
        }
    }
}

pub struct TrendMeta {
    pub source: TrendMetaSource,
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
        MetaHelper::get_trend_commit_meta(user_id, source_id)
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
                source: TrendMetaSource::from(meta_source.unwrap()),
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

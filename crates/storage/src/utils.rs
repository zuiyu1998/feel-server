use crate::prelude::{CommitMeta, CommitMetaSource, TrendMeta, TrendMetaSource};

pub struct MetaHelper {}

impl MetaHelper {
    pub fn get_article_trend_meta(source_id: i32) -> TrendMeta {
        TrendMeta {
            source: TrendMetaSource::Article,
            source_id,
        }
    }

    pub fn get_trend_commit_meta(user_id: i32, source_id: i32) -> CommitMeta {
        CommitMeta {
            user_id,
            source: CommitMetaSource::Trend,
            source_id,
        }
    }
}

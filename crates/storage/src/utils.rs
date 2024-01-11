use rc_entity::sea_orm::ConnectionTrait;

use crate::{
    prelude::{CommitMeta, CommitMetaSource, Trend, TrendDetail, TrendMeta, TrendMetaSource},
    StorageResult,
};

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

    pub async fn update_trends<C: ConnectionTrait>(
        _conn: &C,
        trends: Vec<Trend>,
    ) -> StorageResult<Vec<TrendDetail>> {
        let mut target = vec![];

        let mut articles: Vec<usize> = vec![];

        trends.into_iter().enumerate().for_each(|(index, trend)| {
            if let Some(meta) = &trend.meta {
                match meta.source {
                    TrendMetaSource::Article => {
                        articles.push(index);
                    }
                }
            }

            target.push(TrendDetail::from_trend(trend));
        });

        Ok(target)
    }
}

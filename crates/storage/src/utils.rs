use std::collections::HashMap;

use crate::{
    prelude::{
        Article, CommitMeta, CommitMetaSource, MetaDetail, Trend, TrendDetail, TrendForm,
        TrendMeta, TrendMetaSource, User,
    },
    StorageResult,
};
use rc_entity::sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait};
use rc_entity::{
    prelude::{ArticleColumn, ArticleEntity, UserBaseColumn, UserBaseEntity},
    sea_orm::QueryFilter,
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

    pub async fn get_user_list<C: ConnectionTrait>(
        conn: &C,
        user_ids: Vec<i32>,
    ) -> StorageResult<Vec<User>> {
        let users = UserBaseEntity::find()
            .filter(UserBaseColumn::Id.is_in(user_ids))
            .all(conn)
            .await?
            .into_iter()
            .map(|model| User::from(model))
            .collect::<Vec<User>>();

        Ok(users)
    }

    pub async fn get_article_list<C: ConnectionTrait>(
        conn: &C,
        article_ids: Vec<i32>,
    ) -> StorageResult<Vec<Article>> {
        let articles = ArticleEntity::find()
            .filter(ArticleColumn::Id.is_in(article_ids))
            .all(conn)
            .await?
            .into_iter()
            .map(|model| Article::from(model))
            .collect::<Vec<Article>>();

        Ok(articles)
    }

    pub async fn update_trends<C: ConnectionTrait>(
        conn: &C,
        trends: Vec<Trend>,
    ) -> StorageResult<Vec<TrendDetail>> {
        let mut target: Vec<TrendDetail> = vec![];

        let mut user_ids: Vec<i32> = vec![];
        let mut article_indexs: Vec<usize> = vec![];

        trends.into_iter().enumerate().for_each(|(index, trend)| {
            if let Some(meta) = &trend.meta {
                match meta.source {
                    TrendMetaSource::Article => {
                        article_indexs.push(index);
                    }
                }
            }

            user_ids.push(trend.user_id);

            target.push(TrendDetail::from_trend(trend));
        });

        let article_ids = article_indexs
            .iter()
            .map(|index| {
                target[*index]
                    .meta
                    .as_ref()
                    .and_then(|meta: &TrendMeta| Some(meta.source_id))
                    .unwrap()
            })
            .collect::<Vec<i32>>();

        let users = Self::get_user_list(conn, user_ids)
            .await?
            .into_iter()
            .map(|user| (user.id, user))
            .collect::<HashMap<i32, User>>();

        let articles = Self::get_article_list(conn, article_ids).await?;

        target.iter_mut().for_each(|trend_detail| {
            trend_detail.user = Some(users.get(&trend_detail.user_id).unwrap().clone());
        });

        article_indexs
            .into_iter()
            .enumerate()
            .for_each(|(article_index, index)| {
                target[index].meta_detail =
                    Some(MetaDetail::Article(articles[article_index].clone()));
            });

        Ok(target)
    }
}

pub trait RelatedThrend {
    fn get_thrend_form(&self) -> TrendForm;
}

use rc_entity::sea_orm::{ActiveModelTrait, ConnectionTrait};

mod dto;

pub use dto::*;

use crate::StorageResult;

pub struct ArticleStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> ArticleStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        ArticleStorage { conn }
    }

    pub async fn create_article(&self, form: ArticleForm) -> StorageResult<Article> {
        let active = form.get_article_active_model();

        let model = active.insert(self.conn).await?;

        let article = Article::from(model);

        Ok(article)
    }
}

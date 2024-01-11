use rc_entity::chrono::NaiveDateTime;
use rc_entity::prelude::{get_now, CommitActiveModel, CommitModel};
use rc_entity::sea_orm::Set;

pub struct CommitMeta {
    pub user_id: i32,
    pub source: String,
    pub source_id: i32,
}

pub struct CommitForm {
    pub user_id: i32,
    pub content: String,
    pub meta: CommitMeta,
}

impl CommitForm {
    pub fn get_commit_active_model(&self) -> CommitActiveModel {
        let mut active: CommitActiveModel = Default::default();

        let now = get_now();

        active.meta_user_id = Set(self.meta.user_id);
        active.meta_soure_id = Set(self.meta.source_id);
        active.meta_source = Set(self.meta.source.clone());

        active.user_id = Set(self.user_id);
        active.content = Set(self.content.clone());
        active.update_at = Set(now.clone());
        active.create_at = Set(now.clone());

        active
    }
}

pub struct Commit {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub meta: CommitMeta,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub like_count: i32,
    pub unlike_count: i32,
}

impl From<CommitModel> for Commit {
    fn from(value: CommitModel) -> Self {
        let CommitModel {
            id,
            user_id,
            content,
            create_at,
            update_at,
            meta_user_id,
            meta_source,
            meta_soure_id,
            like_count,
            unlike_count,
            ..
        } = value;

        let meta = CommitMeta {
            user_id: meta_user_id,
            source: meta_source,
            source_id: meta_soure_id,
        };

        Commit {
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

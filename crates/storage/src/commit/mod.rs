use rc_entity::sea_orm::{ActiveModelTrait, ConnectionTrait};

mod dto;

pub use dto::*;

use crate::StorageResult;

pub struct CommitStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> CommitStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        CommitStorage { conn }
    }

    pub async fn crate_commit(&self, form: CommitForm) -> StorageResult<Commit> {
        let active = form.get_commit_active_model();

        let commit = active.insert(self.conn).await?;

        Ok(Commit::from(commit))
    }
}

use rc_entity::prelude::{get_now, UserLabelActiveModel};
use rc_entity::sea_orm::{ActiveModelTrait, ConnectionTrait, FromQueryResult, Set, Statement};

use crate::DATABASEBACKEND;

mod dto;

pub use dto::*;

use crate::StorageResult;

pub struct CommitStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> CommitStorage<'a, C> {
    pub async fn crate_commit(&self) -> StorageResult<()> {
        todo!()
    }
}

use rc_entity::sea_orm::{ActiveModelTrait, ConnectionTrait};

mod dto;

pub use dto::*;

use crate::StorageResult;

pub struct FollowStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> FollowStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        FollowStorage { conn }
    }
}

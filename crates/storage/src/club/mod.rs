use rc_entity::sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, IntoActiveModel};

mod dto;

pub use dto::*;

use crate::{StorageKind, StorageResult};

pub struct ClubStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> ClubStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        ClubStorage { conn }
    }
}

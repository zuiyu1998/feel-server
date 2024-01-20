use rc_entity::sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, IntoActiveModel};

mod dto;

pub use dto::*;

use crate::{traits::GetActiveModel, StorageKind, StorageResult};

pub struct ClubStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> ClubStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        ClubStorage { conn }
    }

    pub async fn create_club(&self, form: ClubForm) -> StorageResult<Club> {
        let active = form.get_active_model();

        let model = active.insert(self.conn).await?;

        Ok(Club::from(model))
    }
}

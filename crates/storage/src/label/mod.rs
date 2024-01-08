use rc_entity::sea_orm::{ActiveModelTrait, ConnectionTrait};

mod dto;

pub use dto::*;

use crate::StorageResult;

pub struct LabelStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> LabelStorage<'a, C> {
    pub async fn create_form(&self, form: LableForm) -> StorageResult<LabelTemplate> {
        let active = form.get_lable_active_model();

        let model = active.insert(self.conn).await?;

        Ok(LabelTemplate::from(model))
    }
}

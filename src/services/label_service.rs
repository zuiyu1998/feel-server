use crate::{state::State, ServerResult};

use rc_entity::sea_orm::TransactionTrait;
use rc_storage::prelude::{LabelStorage, LabelTemplate, LableForm};

pub struct LabelService<'a> {
    state: &'a State,
}

impl<'a> LabelService<'a> {
    pub fn new(state: &'a State) -> Self {
        LabelService { state }
    }

    pub async fn create_label(&self, form: LableForm) -> ServerResult<LabelTemplate> {
        let beign = self.state.conn.begin().await?;

        let storage = LabelStorage::new(&beign);

        let user = storage.create_form(form).await?;

        beign.commit().await?;

        Ok(user)
    }
}

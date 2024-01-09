use rc_entity::prelude::{get_now, UserLabelActiveModel};
use rc_entity::sea_orm::{ActiveModelTrait, ConnectionTrait, Set, Statement};

use crate::DATABASEBACKEND;

mod dto;

pub use dto::*;

use crate::StorageResult;

pub struct LabelStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> LabelStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        LabelStorage { conn }
    }

    pub async fn add_label(&self, user_id: i32, label_id: i32) -> StorageResult<UserLabel> {
        let mut active: UserLabelActiveModel = Default::default();

        let now = get_now();

        active.user_id = Set(user_id);
        active.label_id = Set(label_id);
        active.sequence = Set(0);
        active.create_at = Set(now.clone());
        active.update_at = Set(now.clone());

        let model = active.insert(self.conn).await?;

        Ok(UserLabel::from(model))
    }

    pub async fn get_user_label_list(&self, user_id: i32) -> StorageResult<Vec<Label>> {
        let sql = Statement::from_sql_and_values(DATABASEBACKEND, "", vec![]);

        let sql = self.conn.query_all(sql).await?;

        todo!()
    }

    pub async fn create_form(&self, form: LableForm) -> StorageResult<LabelTemplate> {
        let active = form.get_lable_active_model();

        let model = active.insert(self.conn).await?;

        Ok(LabelTemplate::from(model))
    }
}

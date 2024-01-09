use rc_entity::prelude::{get_now, UserLabelActiveModel};
use rc_entity::sea_orm::{ActiveModelTrait, ConnectionTrait, FromQueryResult, Set, Statement};

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
        let stmt = Statement::from_sql_and_values(
            DATABASEBACKEND,
            r#"
            select 
            pul.id as id,
            pul.user_id as user_id,
            pul."sequence" as "sequence" ,
            pul.create_at as create_at ,
            pul.update_at as update_at ,
            pl."name" as "name",
            pl.description as description,
            pl.effect as effect
            from pb_user_label pul 
            left join pb_label pl on pl.id = pul.label_id 
            left join pb_user_base pub on pub.id = pul.user_id 
            where user_id = $1
            order by  pul."sequence" asc
        "#,
            vec![user_id.into()],
        );

        let labels = Label::find_by_statement(stmt).all(self.conn).await?;

        Ok(labels)
    }

    pub async fn create_form(&self, form: LableForm) -> StorageResult<LabelTemplate> {
        let active = form.get_lable_active_model();

        let model = active.insert(self.conn).await?;

        Ok(LabelTemplate::from(model))
    }
}

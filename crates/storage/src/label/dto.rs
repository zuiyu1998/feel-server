use rc_entity::chrono::NaiveDateTime;
use rc_entity::prelude::{get_now, LabelActiveModel, LabelModel};
use rc_entity::sea_orm::Set;

pub struct LableForm {
    pub description: String,
    pub name: String,
    pub effect: i64,
}

impl LableForm {
    pub fn get_lable_active_model(&self) -> LabelActiveModel {
        let mut active: LabelActiveModel = Default::default();

        let now = get_now();

        active.name = Set(self.name.clone());
        active.description = Set(self.description.clone());
        active.effect = Set(self.effect.clone());
        active.update_at = Set(now.clone());
        active.create_at = Set(now.clone());

        active
    }
}

pub struct LabelLike {}

pub struct LabelTemplate {
    pub id: i32,
    pub description: String,
    pub name: String,
    pub effect: i64,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

impl From<LabelModel> for LabelTemplate {
    fn from(value: LabelModel) -> Self {
        let LabelModel {
            id,
            description,
            name,
            effect,
            create_at,
            update_at,
            ..
        } = value;

        LabelTemplate {
            id,
            description,
            name,
            effect,
            create_at,
            update_at,
        }
    }
}

pub struct Label {
    pub id: i32,
    pub user_id: i32,
    pub sequence: i32,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub description: String,
    pub name: String,
    pub effect: i64,
    pub links: Vec<LabelLike>,
}

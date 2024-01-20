use crate::traits::GetActiveModel;
use rc_entity::prelude::{get_now, ClubActiveModel, ClubModel};
use rc_entity::{chrono::NaiveDateTime, sea_orm::Set};

pub struct ClubForm {
    pub name: String,
    pub user_id: i32,
    pub cover: String,
}

impl GetActiveModel for ClubForm {
    type ActiveModel = ClubActiveModel;

    fn get_active_model(&self) -> Self::ActiveModel {
        let mut active: ClubActiveModel = Default::default();

        let now = get_now();

        active.create_at = Set(now.clone());
        active.update_at = Set(now.clone());
        active.name = Set(self.name.clone());
        active.cover = Set(self.cover.clone());
        active.user_id = Set(self.user_id);

        active
    }
}

#[derive(Clone)]
pub struct Club {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub cover: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

impl From<ClubModel> for Club {
    fn from(value: ClubModel) -> Self {
        let ClubModel {
            id,
            user_id,
            name,
            cover,
            create_at,
            update_at,
            ..
        } = value;

        Club {
            id,
            name,
            user_id,
            cover,
            create_at,
            update_at,
        }
    }
}

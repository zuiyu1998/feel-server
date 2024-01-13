use rc_entity::prelude::{
    get_now, UserFollowActiveModel, UserFollowDetailActiveModel, UserFollowDetailModel,
    UserFollowModel,
};
use rc_entity::{chrono::NaiveDateTime, sea_orm::Set};

pub struct UserFollowDetail {
    pub id: i32,
    pub user_id: i32,
    pub like_count: i32,
    pub follow_count: i32,
}

impl From<UserFollowDetailModel> for UserFollowDetail {
    fn from(value: UserFollowDetailModel) -> Self {
        let UserFollowDetailModel {
            id,
            user_id,
            like_count,
            follow_count,
            ..
        } = value;

        UserFollowDetail {
            id,
            user_id,
            like_count,
            follow_count,
        }
    }
}

pub struct UserFollowDetailForm {
    pub user_id: i32,
    pub like_count: i32,
    pub follow_count: i32,
}

impl UserFollowDetailForm {
    pub fn get_user_follow_detail_active_model(&self) -> UserFollowDetailActiveModel {
        let mut active: UserFollowDetailActiveModel = Default::default();

        active.user_id = Set(self.user_id);
        active.like_count = Set(self.like_count);
        active.follow_count = Set(self.follow_count);

        active
    }
}

pub struct UserFollowForm {
    pub owner_user_id: i32,
    pub follow_user_id: i32,
}

impl UserFollowForm {
    pub fn get_user_follow_active_model(&self) -> UserFollowActiveModel {
        let mut active: UserFollowActiveModel = Default::default();

        let now = get_now();
        active.owner_user_id = Set(self.owner_user_id);
        active.follow_user_id = Set(self.follow_user_id);
        active.create_at = Set(now);

        active
    }
}

pub struct UserFollow {
    pub id: i32,
    pub owner_user_id: i32,
    pub follow_user_id: i32,
    pub create_at: NaiveDateTime,
}

impl From<UserFollowModel> for UserFollow {
    fn from(value: UserFollowModel) -> Self {
        let UserFollowModel {
            id,
            owner_user_id,
            follow_user_id,
            create_at,
            ..
        } = value;

        UserFollow {
            id,
            owner_user_id,
            follow_user_id,
            create_at,
        }
    }
}

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

    pub async fn create_user_follow(&self, user_id: i32) -> StorageResult<UserFollowDetail> {
        let form = UserFollowForm {
            owner_user_id: user_id,
            follow_user_id: user_id,
        };

        self.crate_follow(form).await?;

        let form: UserFollowDetailForm = UserFollowDetailForm {
            user_id,
            like_count: 1,
            follow_count: 0,
        };
        let detail = self.create_follow_detail(form).await?;

        Ok(detail)
    }

    async fn crate_follow(&self, form: UserFollowForm) -> StorageResult<UserFollow> {
        let active = form.get_user_follow_active_model();

        let model = active.insert(self.conn).await?;

        Ok(UserFollow::from(model))
    }

    async fn create_follow_detail(
        &self,
        form: UserFollowDetailForm,
    ) -> StorageResult<UserFollowDetail> {
        let active = form.get_user_follow_detail_active_model();

        let model = active.insert(self.conn).await?;

        Ok(UserFollowDetail::from(model))
    }
}

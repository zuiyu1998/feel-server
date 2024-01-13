use rc_entity::prelude::{UserFollowDetailEntity, UserFollowDetailModel};
use rc_entity::sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, IntoActiveModel};

mod dto;

pub use dto::*;

use crate::{StorageKind, StorageResult};

pub struct FollowStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> FollowStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        FollowStorage { conn }
    }

    pub async fn add_follow(&self, form: UserFollowForm) -> StorageResult<UserFollowDetail> {
        let user_id = form.owner_user_id;
        let follow_user_id = form.follow_user_id;

        self.crate_follow(form).await?;
        let detail = self.update_follow_detail_like_count(user_id, true).await?;
        self.update_follow_detail_follow_count(follow_user_id, true)
            .await?;

        Ok(UserFollowDetail::from(detail))
    }

    pub async fn update_follow_detail_follow_count(
        &self,
        user_id: i32,
        increase: bool,
    ) -> StorageResult<UserFollowDetailModel> {
        let model = self.find_follow_detail(user_id).await?;

        if model.is_none() {
            return Err(StorageKind::FollowDetailNotFound.into());
        }

        let mut model = model.unwrap();

        if increase {
            model.follow_count += 1;
        } else {
            model.follow_count -= 1;
        }

        let model = model.into_active_model().update(self.conn).await?;

        Ok(model)
    }

    pub async fn update_follow_detail_like_count(
        &self,
        user_id: i32,
        increase: bool,
    ) -> StorageResult<UserFollowDetailModel> {
        let model = self.find_follow_detail(user_id).await?;

        if model.is_none() {
            return Err(StorageKind::FollowDetailNotFound.into());
        }

        let mut model = model.unwrap();

        if increase {
            model.like_count += 1;
        } else {
            model.like_count -= 1;
        }

        let model = model.into_active_model().update(self.conn).await?;

        Ok(model)
    }

    pub async fn find_follow_detail(
        &self,
        user_id: i32,
    ) -> StorageResult<Option<UserFollowDetailModel>> {
        let model = UserFollowDetailEntity::find_by_id(user_id)
            .one(self.conn)
            .await?;

        Ok(model)
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

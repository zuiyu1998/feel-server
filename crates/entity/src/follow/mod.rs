mod user_follow;
mod user_follow_detail;

pub use user_follow::{
    ActiveModel as UserFollowActiveModel, Column as UserFollowColumn, Entity as UserFollowEntity,
    Model as UserFollowModel,
};

pub use user_follow_detail::{
    ActiveModel as UserFollowDetailActiveModel, Column as UserFollowDetailColumn,
    Entity as UserFollowDetailEntity, Model as UserFollowDetailModel,
};

use rc_entity::chrono::NaiveDateTime;

pub struct UserFollowDetail {
    pub id: i32,
    pub user_id: i32,
    pub like_count: i32,
    pub follow_user_id: i32,
}

pub struct UserFollow {
    pub id: i32,
    pub owner_user_id: i32,
    pub follow_user_id: i32,
    pub create_at: NaiveDateTime,
}

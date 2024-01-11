use rc_entity::chrono::NaiveDateTime;

pub struct CommitMeta {
    pub user_id: i32,
    pub source: String,
    pub source_id: i32,
}

pub struct Commit {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub meta: CommitMeta,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub like_count: i32,
    pub unlike_count: i32,
}

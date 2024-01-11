use rc_entity::sea_orm::ConnectionTrait;

mod dto;

pub use dto::*;

pub struct TrendStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> TrendStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        TrendStorage { conn }
    }
}

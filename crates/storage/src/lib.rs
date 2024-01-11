use rc_entity::sea_orm::DatabaseBackend;

mod error;
mod user;

mod commit;
mod label;
mod setting;

pub use error::*;

pub use rc_entity::chrono;

const DATABASEBACKEND: DatabaseBackend = DatabaseBackend::Postgres;

pub mod prelude {

    pub use crate::user::*;

    pub use crate::setting::*;

    pub use crate::label::*;
}

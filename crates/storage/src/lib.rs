use rc_entity::sea_orm::DatabaseBackend;

mod error;
mod user;

mod article;
mod commit;
mod follow;
mod label;
mod setting;
mod trend;

mod utils;

pub use error::*;

pub use rc_entity::chrono;

const DATABASEBACKEND: DatabaseBackend = DatabaseBackend::Postgres;

pub mod prelude {

    pub use crate::user::*;

    pub use crate::setting::*;

    pub use crate::label::*;

    pub use crate::commit::*;

    pub use crate::trend::*;

    pub use crate::article::*;

    pub use crate::utils::RelatedThrend;
}

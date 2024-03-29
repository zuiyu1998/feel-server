use rc_entity::sea_orm::DatabaseBackend;

mod error;
mod user;

mod article;
mod club;
mod commit;
mod follow;
mod label;
mod permission;
mod setting;
mod trend;

mod traits;

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

    pub use crate::follow::*;

    pub use crate::permission::*;

    pub use crate::traits::*;
    pub use crate::utils::RelatedThrend;
}

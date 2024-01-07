pub use sea_orm;

mod setting;
mod user;

pub mod prelude {

    pub use crate::user::*;
}

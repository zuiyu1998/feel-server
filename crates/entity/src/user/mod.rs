mod user_auth;
mod user_base;

pub use user_auth::{
    ActiveModel as UserAuthActiveModel, AuthClass as UserAuthClass, Column as UserAuthColumn,
    Entity as UserAuthEntity, Model as UserAuthModel,
};
pub use user_base::{
    ActiveModel as UserBaseActiveModel, Column as UserBaseColumn, Entity as UserBaseEntity,
    Model as UserBaseModel,
};

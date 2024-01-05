mod user_auth;
mod user_base;

pub use user_auth::{
    ActiveModel as UserAuthActiveModel, AuthClass as UserAuthClass, Entity as UserAuthEntity,
    Model as UserAuthModel,
};
pub use user_base::{
    ActiveModel as UserBaseActiveModel, Entity as UserBaseEntity, Model as UserBaseModel,
};

mod permission;
mod role;
mod user_role;

pub use permission::{
    ActiveModel as PermissionActiveModel, Column as PermissionColumn, Entity as PermissionEntity,
    Model as PermissionModel,
};

pub use role::{
    ActiveModel as RoleActiveModel, Column as RoleColumn, Entity as RoleEntity, Model as RoleModel,
};

pub use user_role::{
    ActiveModel as UserRoleActiveModel, Column as UserRoleColumn, Entity as UserRoleEntity,
    Model as UserRoleModel,
};

mod permission;
mod role;

pub use permission::{
    ActiveModel as PermissionActiveModel, Column as PermissionColumn, Entity as PermissionEntity,
    Model as PermissionModel,
};

pub use role::{
    ActiveModel as RoleActiveModel, Column as RoleColumn, Entity as RoleEntity, Model as RoleModel,
};

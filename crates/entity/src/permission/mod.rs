mod permission;
mod role;
mod role_permission;
mod url_permission;
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

pub use role_permission::{
    ActiveModel as RolePermissionActiveModel, Column as RolePermissionColumn,
    Entity as RolePermissionEntity, Model as RolePermissionModel,
};

pub use url_permission::{
    ActiveModel as UrlPermissionActiveModel, Column as UrlPermissionColumn,
    Entity as UrlPermissionEntity, Model as UrlPermissionModel,
};

use rc_entity::prelude::{RolePermissionColumn, RolePermissionEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RolePermissionEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RolePermissionColumn::Id)
                            .integer()
                            .unique_key()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RolePermissionColumn::PermissionId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RolePermissionColumn::RoleId)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RolePermissionEntity).to_owned())
            .await
    }
}

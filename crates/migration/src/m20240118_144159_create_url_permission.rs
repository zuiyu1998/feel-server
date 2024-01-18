use rc_entity::prelude::{UrlPermissionColumn, UrlPermissionEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UrlPermissionEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UrlPermissionColumn::Id)
                            .integer()
                            .unique_key()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UrlPermissionColumn::PermissionId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UrlPermissionColumn::Url)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UrlPermissionColumn::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UrlPermissionColumn::UpdateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UrlPermissionColumn::IsDelete)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(UrlPermissionColumn::IsEnable)
                            .boolean()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UrlPermissionEntity).to_owned())
            .await
    }
}

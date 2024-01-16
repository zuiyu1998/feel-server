use rc_entity::prelude::{RoleColumn, RoleEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RoleEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RoleColumn::Id)
                            .integer()
                            .unique_key()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RoleColumn::Name)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(RoleColumn::CreateAt).date_time().not_null())
                    .col(ColumnDef::new(RoleColumn::UpdateAt).date_time().not_null())
                    .col(
                        ColumnDef::new(RoleColumn::IsDelete)
                            .boolean()
                            .default(false),
                    )
                    .col(ColumnDef::new(RoleColumn::IsEnable).boolean().default(true))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RoleEntity).to_owned())
            .await
    }
}

use rc_entity::prelude::{UserRoleColumn, UserRoleEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRoleEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRoleColumn::Id)
                            .integer()
                            .unique_key()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserRoleColumn::UserId).integer().not_null())
                    .col(ColumnDef::new(UserRoleColumn::RoleId).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRoleEntity).to_owned())
            .await
    }
}

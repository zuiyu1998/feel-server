use sea_orm_migration::prelude::*;

use rc_entity::prelude::{UserBaseColumn, UserBaseEntity};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserBaseEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserBaseColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserBaseColumn::Nikename)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(UserBaseColumn::Uid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(UserBaseColumn::Avatar).string().not_null())
                    .col(
                        ColumnDef::new(UserBaseColumn::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserBaseColumn::UpdateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserBaseColumn::IsDelete)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(UserBaseColumn::IsEnable)
                            .boolean()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserBaseEntity).to_owned())
            .await
    }
}

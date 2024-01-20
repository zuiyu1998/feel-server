use rc_entity::prelude::{ClubColumn, ClubEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ClubEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ClubColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ClubColumn::UserId).integer().not_null())
                    .col(ColumnDef::new(ClubColumn::Name).string())
                    .col(ColumnDef::new(ClubColumn::Cover).string())
                    .col(ColumnDef::new(ClubColumn::CreateAt).date_time().not_null())
                    .col(ColumnDef::new(ClubColumn::UpdateAt).date_time().not_null())
                    .col(
                        ColumnDef::new(ClubColumn::IsDelete)
                            .boolean()
                            .default(false),
                    )
                    .col(ColumnDef::new(ClubColumn::IsEnable).boolean().default(true))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ClubEntity).to_owned())
            .await
    }
}

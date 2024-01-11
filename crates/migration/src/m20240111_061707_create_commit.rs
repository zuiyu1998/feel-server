use rc_entity::prelude::{CommitColumn, CommitEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CommitEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CommitColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CommitColumn::UserId).integer().not_null())
                    .col(ColumnDef::new(CommitColumn::LikeCount).integer().not_null())
                    .col(ColumnDef::new(CommitColumn::MetaSource).string().not_null())
                    .col(
                        ColumnDef::new(CommitColumn::MetaSoureId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CommitColumn::MetaUserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CommitColumn::UnlikeCount)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CommitColumn::Content).string().not_null())
                    .col(
                        ColumnDef::new(CommitColumn::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CommitColumn::UpdateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CommitColumn::IsDelete)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(CommitColumn::IsEnable)
                            .boolean()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CommitEntity).to_owned())
            .await
    }
}

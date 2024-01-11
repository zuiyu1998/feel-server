use rc_entity::prelude::{TrendColumn, TrendEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TrendEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TrendColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TrendColumn::UserId).integer().not_null())
                    .col(ColumnDef::new(TrendColumn::LikeCount).integer().not_null())
                    .col(ColumnDef::new(TrendColumn::MetaSource).string().not_null())
                    .col(
                        ColumnDef::new(TrendColumn::MetaSoureId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TrendColumn::UnlikeCount)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TrendColumn::Content).string().not_null())
                    .col(ColumnDef::new(TrendColumn::CreateAt).date_time().not_null())
                    .col(ColumnDef::new(TrendColumn::UpdateAt).date_time().not_null())
                    .col(
                        ColumnDef::new(TrendColumn::IsDelete)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(TrendColumn::IsEnable)
                            .boolean()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TrendEntity).to_owned())
            .await
    }
}

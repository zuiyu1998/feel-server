use rc_entity::prelude::{ArticleColumn, ArticleEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ArticleEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ArticleColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ArticleColumn::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(ArticleColumn::LikeCount)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ArticleColumn::Title).string())
                    .col(ColumnDef::new(ArticleColumn::Background).string())
                    .col(
                        ColumnDef::new(ArticleColumn::UnlikeCount)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ArticleColumn::Content).string().not_null())
                    .col(
                        ColumnDef::new(ArticleColumn::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ArticleColumn::UpdateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ArticleColumn::IsDelete)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(ArticleColumn::IsEnable)
                            .boolean()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ArticleEntity).to_owned())
            .await
    }
}

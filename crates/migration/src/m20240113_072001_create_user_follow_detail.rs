use rc_entity::prelude::{UserFollowDetailColumn, UserFollowDetailEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserFollowDetailEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserFollowDetailColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserFollowDetailColumn::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserFollowDetailColumn::LikeCount)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserFollowDetailColumn::FollowCount)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserFollowDetailEntity).to_owned())
            .await
    }
}

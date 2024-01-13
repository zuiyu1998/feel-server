use rc_entity::prelude::{UserFollowColumn, UserFollowEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserFollowEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserFollowColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserFollowColumn::OwnerUserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserFollowColumn::FollowUserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserFollowColumn::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserFollowColumn::IsDelete)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(UserFollowColumn::IsEnable)
                            .boolean()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserFollowEntity).to_owned())
            .await
    }
}

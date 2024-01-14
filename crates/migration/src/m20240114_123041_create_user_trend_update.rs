use rc_entity::prelude::{TrendUpdateColumn, TrendUpdateEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TrendUpdateEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TrendUpdateColumn::UserId)
                            .integer()
                            .unique_key()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TrendUpdateColumn::UpdateAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TrendUpdateEntity).to_owned())
            .await
    }
}

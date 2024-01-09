use rc_entity::prelude::{UserLabelColumn, UserLabelEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserLabelEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserLabelColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserLabelColumn::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(UserLabelColumn::Sequence)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserLabelColumn::LabelId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserLabelColumn::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserLabelColumn::UpdateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserLabelColumn::IsDelete)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(UserLabelColumn::IsEnable)
                            .boolean()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserLabelEntity).to_owned())
            .await
    }
}

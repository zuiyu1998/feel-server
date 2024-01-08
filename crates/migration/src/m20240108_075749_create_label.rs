use rc_entity::prelude::{LabelColumn, LabelEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LabelEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LabelColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LabelColumn::Description).string().not_null())
                    .col(
                        ColumnDef::new(LabelColumn::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(LabelColumn::Effect).big_integer().not_null())
                    .col(ColumnDef::new(LabelColumn::CreateAt).date_time().not_null())
                    .col(ColumnDef::new(LabelColumn::UpdateAt).date_time().not_null())
                    .col(
                        ColumnDef::new(LabelColumn::IsDelete)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(LabelColumn::IsEnable)
                            .boolean()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LabelEntity).to_owned())
            .await
    }
}

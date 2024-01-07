use rc_entity::prelude::{SettingColumn, SettingEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SettingEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SettingColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SettingColumn::Key).string().not_null())
                    .col(ColumnDef::new(SettingColumn::RawData).string().not_null())
                    .col(
                        ColumnDef::new(SettingColumn::SettingDataType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SettingColumn::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SettingColumn::UpdateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SettingColumn::IsDelete)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(SettingColumn::IsEnable)
                            .boolean()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SettingEntity).to_owned())
            .await
    }
}

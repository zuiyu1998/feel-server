use rc_entity::prelude::{UserAuthColumn, UserAuthEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserAuthEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserAuthColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserAuthColumn::UserId).integer())
                    .col(
                        ColumnDef::new(UserAuthColumn::AuthClass)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserAuthColumn::UniqueName)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserAuthColumn::AuthData).binary().not_null())
                    .col(
                        ColumnDef::new(UserAuthColumn::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserAuthColumn::UpdateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserAuthColumn::IsDelete)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(UserAuthColumn::IsEnable)
                            .boolean()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserAuthEntity).to_owned())
            .await
    }
}

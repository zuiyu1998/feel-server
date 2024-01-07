use rc_entity::{
    prelude::{get_now, SettingActiveModel, SettingColumn, SettingDataType, SettingEntity},
    sea_orm::{EntityTrait, QueryTrait, Set},
};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

async fn create_avatars<'c>(manager: &SchemaManager<'c>) -> Result<(), DbErr> {
    let avatars: Vec<String> = vec![
        "https://i1.hdslb.com/bfs/face/9c5f14d6749daded668f3f66998baf4a50e7d8da.png".to_string(),
    ];

    let conn = manager.get_connection();

    let now = get_now();

    let mut active: SettingActiveModel = Default::default();

    active.key = Set("avatar".to_string());
    active.create_at = Set(now.clone());
    active.update_at = Set(now.clone());

    let json_value = serde_json::Value::from(avatars).to_string();

    active.setting_data_type = Set(SettingDataType::Array);
    active.raw_data = Set(json_value);

    let sql = SettingEntity::insert(active).build(sea_orm::DatabaseBackend::Postgres);

    conn.query_one(sql).await?;
    Ok(())
}

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
            .await?;

        create_avatars(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SettingEntity).to_owned())
            .await
    }
}

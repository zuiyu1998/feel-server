pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_base;
mod m20240105_032645_create_user_auth;
mod m20240107_132002_create_setting;
mod m20240108_075749_create_label;
mod m20240108_080154_create_user_label;
mod m20240111_061707_create_commit;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_base::Migration),
            Box::new(m20240105_032645_create_user_auth::Migration),
            Box::new(m20240107_132002_create_setting::Migration),
            Box::new(m20240108_075749_create_label::Migration),
            Box::new(m20240108_080154_create_user_label::Migration),
            Box::new(m20240111_061707_create_commit::Migration),
        ]
    }
}

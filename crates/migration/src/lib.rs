pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_base;
mod m20240105_032645_create_user_auth;
mod m20240107_132002_create_setting;
mod m20240108_075749_create_label;
mod m20240108_080154_create_user_label;
mod m20240111_061707_create_commit;
mod m20240111_073155_create_trend;
mod m20240112_061921_create_article;
mod m20240113_072001_create_user_follow_detail;
mod m20240113_072006_create_user_follow;
mod m20240114_123041_create_user_trend_update;
mod m20240116_054141_create_permission;
mod m20240116_055748_create_role;
mod m20240116_060328_create_user_role;
mod m20240116_061300_create_role_permission;
mod m20240118_144159_create_url_permission;

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
            Box::new(m20240111_073155_create_trend::Migration),
            Box::new(m20240112_061921_create_article::Migration),
            Box::new(m20240113_072001_create_user_follow_detail::Migration),
            Box::new(m20240113_072006_create_user_follow::Migration),
            Box::new(m20240114_123041_create_user_trend_update::Migration),
            Box::new(m20240116_054141_create_permission::Migration),
            Box::new(m20240116_055748_create_role::Migration),
            Box::new(m20240116_060328_create_user_role::Migration),
            Box::new(m20240116_061300_create_role_permission::Migration),
            Box::new(m20240118_144159_create_url_permission::Migration),
        ]
    }
}

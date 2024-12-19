pub use sea_orm_migration::prelude::*;

mod m20241214_112002_users_table;
mod m20241214_200600_donations_table;
mod m20241215_143758_claims_table;
mod m20241218_213842_reviews_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241214_112002_users_table::Migration),
            Box::new(m20241214_200600_donations_table::Migration),
            Box::new(m20241215_143758_claims_table::Migration),
            Box::new(m20241218_213842_reviews_table::Migration),
        ]
    }
}

pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users_table;
mod m20220723_143827_create_permissions_table;
mod tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users_table::Migration),
            Box::new(m20220723_143827_create_permissions_table::Migration),
        ]
    }
}
